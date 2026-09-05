//! RAW10 packed ↔ canonical boundary adapters (§18).
//!
//! **Boundary representation, NOT canonical identity.** The canonical sensor
//! sample is a 10-bit value ([`SensorSample`], `0..=1023`). RAW10 packing is a
//! *transport* byte layout: four 10-bit samples packed into five bytes. This
//! module provides explicit, tested adapters `packed → canonical → packed`
//! without requiring a full frame to be expanded to `u16` for convenience.
//!
//! ## RAW10 pack layout (little-endian MSB byte)
//!
//! Each byte-aligned "packet" holds **4 samples** in **5 bytes**:
//!
//! ```text
//!  byte 0:  s0[7:0]                                    (8 LSBs of sample 0)
//!  byte 1:  s1[7:0]
//!  byte 2:  s2[7:0]
//!  byte 3:  s3[7:0]
//!  byte 4:  0 0  s3[9:8] s2[9:8] s1[9:8] s0[9:8]      (top nibble = 0)
//!                                                       sample 0 MSBs in bits 0..1
//! ```
//!
//! 4 samples × 10 bits = 40 bits = 5 bytes exactly. The high ("combining")
//! byte packs the two MSBs of each sample in **sample order from the least
//! significant bits upward**: sample 0's MSBs are bits `1..0`, sample 1's
//! `3..2`, sample 2's `5..4`, sample 3's `7..6`, and the top two bits are zero.
//!
//! This is the common CSI-2 RAW10 arrangement (and the media-bus packing the
//! IMX415 driver exposes). The layout — endianness, active bits, row order,
//! stride absence, CFA origin — is declared explicitly here rather than
//! inferred from a host struct (§17). A device with different packetization
//! needs a *different* adapter, never a silent reinterpretation.

use crate::domain::{SensorSample, SENSOR_MAX_VALUE};
use crate::error::{CameraError, CameraResult};

/// Samples per RAW10 packing packet.
pub const RAW10_SAMPLES_PER_PACK: usize = 4;
/// Bytes per RAW10 packing packet.
pub const RAW10_BYTES_PER_PACK: usize = 5;

/// Pack a complete, byte-aligned slice of canonical samples into RAW10 bytes.
///
/// `samples.len()` must be a multiple of [`RAW10_SAMPLES_PER_PACK`]. Returns
/// `samples.len() * 5 / 4` bytes. For large frames, use [`Raw10Packer`]
/// incrementally to avoid a full-frame intermediate.
pub fn pack_raw10(samples: &[SensorSample]) -> CameraResult<Vec<u8>> {
    if samples.len() % RAW10_SAMPLES_PER_PACK != 0 {
        return Err(CameraError::Malformed(
            "RAW10 pack requires a sample count divisible by 4",
        ));
    }
    let mut p = Raw10Packer::with_capacity(samples.len());
    for &s in samples {
        p.push(s)?;
    }
    Ok(p.into_bytes())
}

/// Unpack a complete, byte-aligned RAW10 buffer into canonical samples.
///
/// `bytes.len()` must be a multiple of [`RAW10_BYTES_PER_PACK`]. Returns
/// `bytes.len() * 4 / 5` samples.
pub fn unpack_raw10(bytes: &[u8]) -> CameraResult<Vec<SensorSample>> {
    if bytes.len() % RAW10_BYTES_PER_PACK != 0 {
        return Err(CameraError::Malformed(
            "RAW10 unpack requires a byte count divisible by 5",
        ));
    }
    let n = bytes.len() / RAW10_BYTES_PER_PACK * RAW10_SAMPLES_PER_PACK;
    let mut out = Vec::with_capacity(n);
    for chunk in bytes.chunks_exact(RAW10_BYTES_PER_PACK) {
        let s0 = (chunk[0] as u16) | (((chunk[4] as u16) & 0b0000_0011) << 8);
        let s1 = (chunk[1] as u16) | (((chunk[4] as u16) & 0b0000_1100) << 6);
        let s2 = (chunk[2] as u16) | (((chunk[4] as u16) & 0b0011_0000) << 4);
        let s3 = (chunk[3] as u16) | (((chunk[4] as u16) & 0b1100_0000) << 2);
        for v in [s0, s1, s2, s3] {
            out.push(SensorSample::new(v)?);
        }
    }
    Ok(out)
}

/// Incremental RAW10 packer: pushes canonical samples in raster order and emits
/// packed bytes without buffering a whole frame.
#[derive(Clone, Debug)]
pub struct Raw10Packer {
    bytes: Vec<u8>,
    /// The four low bytes of the in-progress packet (index = sample index).
    low: [u8; RAW10_SAMPLES_PER_PACK],
    /// The two MSBs of each sample in the in-progress packet.
    high: [u8; RAW10_SAMPLES_PER_PACK],
    filled: usize,
}

impl Default for Raw10Packer {
    fn default() -> Self {
        Self {
            bytes: Vec::new(),
            low: [0; RAW10_SAMPLES_PER_PACK],
            high: [0; RAW10_SAMPLES_PER_PACK],
            filled: 0,
        }
    }
}

impl Raw10Packer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Pre-reserve backing storage for `sample_count` samples (5/4 ratio).
    pub fn with_capacity(sample_count: usize) -> Self {
        let mut p = Self::new();
        p.bytes
            .reserve(sample_count / RAW10_SAMPLES_PER_PACK * RAW10_BYTES_PER_PACK);
        p
    }

    /// Push one canonical sample in raster order.
    pub fn push(&mut self, sample: SensorSample) -> CameraResult<()> {
        let v = sample.value();
        debug_assert!(v <= SENSOR_MAX_VALUE);
        let i = self.filled;
        self.low[i] = (v & 0xFF) as u8;
        self.high[i] = ((v >> 8) & 0b11) as u8;
        self.filled += 1;
        if self.filled == RAW10_SAMPLES_PER_PACK {
            self.flush_packet();
        }
        Ok(())
    }

    fn flush_packet(&mut self) {
        self.bytes.extend_from_slice(&self.low);
        let comb = (self.high[0] & 0b11)
            | ((self.high[1] & 0b11) << 2)
            | ((self.high[2] & 0b11) << 4)
            | ((self.high[3] & 0b11) << 6);
        self.bytes.push(comb);
        self.filled = 0;
        self.low = [0; RAW10_SAMPLES_PER_PACK];
        self.high = [0; RAW10_SAMPLES_PER_PACK];
    }

    /// Consume the packer, returning the packed bytes.
    ///
    /// `filled` must be 0 (stream aligned); a partial packet is a programming
    /// error and is reported by the caller before this point.
    pub fn into_bytes(self) -> Vec<u8> {
        debug_assert_eq!(self.filled, 0, "RAW10 stream ended on a partial packet");
        self.bytes
    }
}

/// Incremental RAW10 reader: yields canonical samples in raster order without
/// expanding a whole frame.
#[derive(Clone, Debug)]
pub struct Raw10Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
    /// Remaining samples already decoded from the current packet.
    pending: std::vec::IntoIter<SensorSample>,
}

impl<'a> Raw10Reader<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Raw10Reader {
            bytes,
            pos: 0,
            pending: Vec::new().into_iter(),
        }
    }

    /// Read the next canonical sample, or `None` at end of stream.
    pub fn next(&mut self) -> CameraResult<Option<SensorSample>> {
        if let Some(s) = self.pending.next() {
            return Ok(Some(s));
        }
        if self.pos == self.bytes.len() {
            return Ok(None);
        }
        if self.bytes.len() - self.pos < RAW10_BYTES_PER_PACK {
            return Err(CameraError::Malformed(
                "RAW10 stream has a trailing partial packet",
            ));
        }
        let chunk = &self.bytes[self.pos..self.pos + RAW10_BYTES_PER_PACK];
        self.pos += RAW10_BYTES_PER_PACK;
        let s0 = (chunk[0] as u16) | (((chunk[4] as u16) & 0b0000_0011) << 8);
        let s1 = (chunk[1] as u16) | (((chunk[4] as u16) & 0b0000_1100) << 6);
        let s2 = (chunk[2] as u16) | (((chunk[4] as u16) & 0b0011_0000) << 4);
        let s3 = (chunk[3] as u16) | (((chunk[4] as u16) & 0b1100_0000) << 2);
        let a = SensorSample::new(s0)?;
        let b = SensorSample::new(s1)?;
        let c = SensorSample::new(s2)?;
        let d = SensorSample::new(s3)?;
        self.pending = vec![a, b, c, d].into_iter();
        Ok(self.pending.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn samples(vals: &[u16]) -> Vec<SensorSample> {
        vals.iter()
            .map(|&v| SensorSample::new(v).unwrap())
            .collect()
    }

    #[test]
    fn pack_layout_is_declared_bit_exact() {
        // One packet of [0x1, 0x2, 0x3, 0x3FF]:
        //  s0=0x001 -> low 0x01, high 00 -> bits 0..1 = 00
        //  s1=0x002 -> low 0x02, high 00 -> bits 2..3 = 00
        //  s2=0x003 -> low 0x03, high 00 -> bits 4..5 = 00
        //  s3=0x3FF -> low 0xFF, high 11 -> bits 6..7 = 11
        //  combining byte = 0b11000000 = 0xC0
        let packed = pack_raw10(&samples(&[0x001, 0x002, 0x003, 0x3FF])).unwrap();
        assert_eq!(packed, vec![0x01, 0x02, 0x03, 0xFF, 0xC0]);
    }

    #[test]
    fn top_two_lsb_and_msb_carry_exactly() {
        // Sample 0x2AA = 0b10_1010_1010 -> low 0xAA, high 0b10.
        // Blended with others to exercise every bit of the combining byte.
        let vals = [0x2AA, 0x155, 0x000, 0x3FF];
        let packed = pack_raw10(&samples(&vals)).unwrap();
        let back = unpack_raw10(&packed).unwrap();
        let got: Vec<u16> = back.iter().map(|s| s.value()).collect();
        assert_eq!(got, vals);
    }

    #[test]
    fn round_trip_many_packets() {
        let mut vals = Vec::new();
        for i in 0..64u16 {
            vals.push((i * 17) & 0x3FF);
        }
        let packed = pack_raw10(&samples(&vals)).unwrap();
        let back = unpack_raw10(&packed).unwrap();
        let got: Vec<u16> = back.iter().map(|s| s.value()).collect();
        assert_eq!(got, vals);
    }

    #[test]
    fn incremental_packer_matches_bulk() {
        let vals: Vec<u16> = (0..64u16).map(|i| (i * 37 + 5) & 0x3FF).collect();
        let bulk = pack_raw10(&samples(&vals)).unwrap();
        let mut p = Raw10Packer::new();
        for &v in &vals {
            p.push(SensorSample::new(v).unwrap()).unwrap();
        }
        assert_eq!(p.into_bytes(), bulk);
    }

    #[test]
    fn incremental_reader_matches_bulk() {
        let vals: Vec<u16> = (0..48u16).map(|i| (i * 53) & 0x3FF).collect();
        let packed = pack_raw10(&samples(&vals)).unwrap();
        let mut r = Raw10Reader::new(&packed);
        let mut got = Vec::new();
        while let Some(s) = r.next().unwrap() {
            got.push(s.value());
        }
        assert_eq!(got, vals);
    }

    #[test]
    fn rejects_unfilled_pack() {
        assert!(pack_raw10(&samples(&[1, 2, 3])).is_err());
    }

    #[test]
    fn rejects_partial_unpack() {
        assert!(unpack_raw10(&[0, 0, 0, 0]).is_err());
    }

    #[test]
    fn reader_rejects_trailing_partial() {
        let mut r = Raw10Reader::new(&[0, 0, 0]);
        assert!(r.next().is_err());
    }

    #[test]
    fn full_range_boundary_values_round_trip() {
        let vals = [0u16, 1, 1022, 1023];
        let packed = pack_raw10(&samples(&vals)).unwrap();
        let back = unpack_raw10(&packed).unwrap();
        // Canonical identity is preserved exactly for the full 0..=1023 range.
        let got: Vec<u16> = back.iter().map(|s| s.value()).collect();
        assert_eq!(got, vals);
        assert!(!(back.iter().any(|s| s.value() > SENSOR_MAX_VALUE)));
    }
}
