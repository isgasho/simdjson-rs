use crate::charutils::*;
use crate::unlikely;
use crate::*;
#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

const POWER_OF_TEN: [f64; 617] = [
    1e-308, 1e-307, 1e-306, 1e-305, 1e-304, 1e-303, 1e-302, 1e-301, 1e-300, 1e-299, 1e-298, 1e-297,
    1e-296, 1e-295, 1e-294, 1e-293, 1e-292, 1e-291, 1e-290, 1e-289, 1e-288, 1e-287, 1e-286, 1e-285,
    1e-284, 1e-283, 1e-282, 1e-281, 1e-280, 1e-279, 1e-278, 1e-277, 1e-276, 1e-275, 1e-274, 1e-273,
    1e-272, 1e-271, 1e-270, 1e-269, 1e-268, 1e-267, 1e-266, 1e-265, 1e-264, 1e-263, 1e-262, 1e-261,
    1e-260, 1e-259, 1e-258, 1e-257, 1e-256, 1e-255, 1e-254, 1e-253, 1e-252, 1e-251, 1e-250, 1e-249,
    1e-248, 1e-247, 1e-246, 1e-245, 1e-244, 1e-243, 1e-242, 1e-241, 1e-240, 1e-239, 1e-238, 1e-237,
    1e-236, 1e-235, 1e-234, 1e-233, 1e-232, 1e-231, 1e-230, 1e-229, 1e-228, 1e-227, 1e-226, 1e-225,
    1e-224, 1e-223, 1e-222, 1e-221, 1e-220, 1e-219, 1e-218, 1e-217, 1e-216, 1e-215, 1e-214, 1e-213,
    1e-212, 1e-211, 1e-210, 1e-209, 1e-208, 1e-207, 1e-206, 1e-205, 1e-204, 1e-203, 1e-202, 1e-201,
    1e-200, 1e-199, 1e-198, 1e-197, 1e-196, 1e-195, 1e-194, 1e-193, 1e-192, 1e-191, 1e-190, 1e-189,
    1e-188, 1e-187, 1e-186, 1e-185, 1e-184, 1e-183, 1e-182, 1e-181, 1e-180, 1e-179, 1e-178, 1e-177,
    1e-176, 1e-175, 1e-174, 1e-173, 1e-172, 1e-171, 1e-170, 1e-169, 1e-168, 1e-167, 1e-166, 1e-165,
    1e-164, 1e-163, 1e-162, 1e-161, 1e-160, 1e-159, 1e-158, 1e-157, 1e-156, 1e-155, 1e-154, 1e-153,
    1e-152, 1e-151, 1e-150, 1e-149, 1e-148, 1e-147, 1e-146, 1e-145, 1e-144, 1e-143, 1e-142, 1e-141,
    1e-140, 1e-139, 1e-138, 1e-137, 1e-136, 1e-135, 1e-134, 1e-133, 1e-132, 1e-131, 1e-130, 1e-129,
    1e-128, 1e-127, 1e-126, 1e-125, 1e-124, 1e-123, 1e-122, 1e-121, 1e-120, 1e-119, 1e-118, 1e-117,
    1e-116, 1e-115, 1e-114, 1e-113, 1e-112, 1e-111, 1e-110, 1e-109, 1e-108, 1e-107, 1e-106, 1e-105,
    1e-104, 1e-103, 1e-102, 1e-101, 1e-100, 1e-99, 1e-98, 1e-97, 1e-96, 1e-95, 1e-94, 1e-93, 1e-92,
    1e-91, 1e-90, 1e-89, 1e-88, 1e-87, 1e-86, 1e-85, 1e-84, 1e-83, 1e-82, 1e-81, 1e-80, 1e-79,
    1e-78, 1e-77, 1e-76, 1e-75, 1e-74, 1e-73, 1e-72, 1e-71, 1e-70, 1e-69, 1e-68, 1e-67, 1e-66,
    1e-65, 1e-64, 1e-63, 1e-62, 1e-61, 1e-60, 1e-59, 1e-58, 1e-57, 1e-56, 1e-55, 1e-54, 1e-53,
    1e-52, 1e-51, 1e-50, 1e-49, 1e-48, 1e-47, 1e-46, 1e-45, 1e-44, 1e-43, 1e-42, 1e-41, 1e-40,
    1e-39, 1e-38, 1e-37, 1e-36, 1e-35, 1e-34, 1e-33, 1e-32, 1e-31, 1e-30, 1e-29, 1e-28, 1e-27,
    1e-26, 1e-25, 1e-24, 1e-23, 1e-22, 1e-21, 1e-20, 1e-19, 1e-18, 1e-17, 1e-16, 1e-15, 1e-14,
    1e-13, 1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1e0, 1e1,
    1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15, 1e16, 1e17, 1e18,
    1e19, 1e20, 1e21, 1e22, 1e23, 1e24, 1e25, 1e26, 1e27, 1e28, 1e29, 1e30, 1e31, 1e32, 1e33, 1e34,
    1e35, 1e36, 1e37, 1e38, 1e39, 1e40, 1e41, 1e42, 1e43, 1e44, 1e45, 1e46, 1e47, 1e48, 1e49, 1e50,
    1e51, 1e52, 1e53, 1e54, 1e55, 1e56, 1e57, 1e58, 1e59, 1e60, 1e61, 1e62, 1e63, 1e64, 1e65, 1e66,
    1e67, 1e68, 1e69, 1e70, 1e71, 1e72, 1e73, 1e74, 1e75, 1e76, 1e77, 1e78, 1e79, 1e80, 1e81, 1e82,
    1e83, 1e84, 1e85, 1e86, 1e87, 1e88, 1e89, 1e90, 1e91, 1e92, 1e93, 1e94, 1e95, 1e96, 1e97, 1e98,
    1e99, 1e100, 1e101, 1e102, 1e103, 1e104, 1e105, 1e106, 1e107, 1e108, 1e109, 1e110, 1e111,
    1e112, 1e113, 1e114, 1e115, 1e116, 1e117, 1e118, 1e119, 1e120, 1e121, 1e122, 1e123, 1e124,
    1e125, 1e126, 1e127, 1e128, 1e129, 1e130, 1e131, 1e132, 1e133, 1e134, 1e135, 1e136, 1e137,
    1e138, 1e139, 1e140, 1e141, 1e142, 1e143, 1e144, 1e145, 1e146, 1e147, 1e148, 1e149, 1e150,
    1e151, 1e152, 1e153, 1e154, 1e155, 1e156, 1e157, 1e158, 1e159, 1e160, 1e161, 1e162, 1e163,
    1e164, 1e165, 1e166, 1e167, 1e168, 1e169, 1e170, 1e171, 1e172, 1e173, 1e174, 1e175, 1e176,
    1e177, 1e178, 1e179, 1e180, 1e181, 1e182, 1e183, 1e184, 1e185, 1e186, 1e187, 1e188, 1e189,
    1e190, 1e191, 1e192, 1e193, 1e194, 1e195, 1e196, 1e197, 1e198, 1e199, 1e200, 1e201, 1e202,
    1e203, 1e204, 1e205, 1e206, 1e207, 1e208, 1e209, 1e210, 1e211, 1e212, 1e213, 1e214, 1e215,
    1e216, 1e217, 1e218, 1e219, 1e220, 1e221, 1e222, 1e223, 1e224, 1e225, 1e226, 1e227, 1e228,
    1e229, 1e230, 1e231, 1e232, 1e233, 1e234, 1e235, 1e236, 1e237, 1e238, 1e239, 1e240, 1e241,
    1e242, 1e243, 1e244, 1e245, 1e246, 1e247, 1e248, 1e249, 1e250, 1e251, 1e252, 1e253, 1e254,
    1e255, 1e256, 1e257, 1e258, 1e259, 1e260, 1e261, 1e262, 1e263, 1e264, 1e265, 1e266, 1e267,
    1e268, 1e269, 1e270, 1e271, 1e272, 1e273, 1e274, 1e275, 1e276, 1e277, 1e278, 1e279, 1e280,
    1e281, 1e282, 1e283, 1e284, 1e285, 1e286, 1e287, 1e288, 1e289, 1e290, 1e291, 1e292, 1e293,
    1e294, 1e295, 1e296, 1e297, 1e298, 1e299, 1e300, 1e301, 1e302, 1e303, 1e304, 1e305, 1e306,
    1e307, 1e308,
];

//#[inline(always)]
#[cfg_attr(not(feature = "no-inline"), inline(always))]
pub fn is_integer(c: u8) -> bool {
    // this gets compiled to (uint8_t)(c - '0') <= 9 on all decent compilers
    c >= b'0' && c <= b'9'
}

// We need to check that the character following a zero is valid. This is
// probably frequent and it is hard than it looks. We are building all of this
// just to differentiate between 0x1 (invalid), 0,1 (valid) 0e1 (valid)...
const STRUCTURAL_OR_WHITESPACE_OR_EXPONENT_OR_DECIMAL_NEGATED: [bool; 256] = [
    false, true, true, true, true, true, true, true, true, false, false, true, true, false, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, false, true, true, true, true, true, true, true, true, true, true, true, false, true,
    false, true, true, true, true, true, true, true, true, true, true, true, false, true, true,
    true, true, true, true, true, true, true, true, false, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    false, true, false, true, true, true, true, true, true, true, false, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, false, true, false, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true,
];

#[cfg_attr(not(feature = "no-inline"), inline(always))]
fn is_not_structural_or_whitespace_or_exponent_or_decimal(c: u8) -> bool {
    STRUCTURAL_OR_WHITESPACE_OR_EXPONENT_OR_DECIMAL_NEGATED[c as usize]
}

//#define SWAR_NUMBER_PARSING

//#ifdef SWAR_NUMBER_PARSING

// #ifdef _MSC_VER
// check quickly whether the next 8 chars are made of digits
// at a glance, it looks better than Mula's
// http://0x80.pl/articles/swar-digits-validate.html
#[cfg_attr(not(feature = "no-inline"), inline)]
fn is_made_of_eight_digits_fast(chars: &[u8]) -> bool {
    let val: u64 = unsafe { *(chars[0..8].as_ptr() as *const u64) };

    //    let val: __m64 = *(chars as *const __m64);
    // a branchy method might be faster:
    // return (( val & 0xF0F0F0F0F0F0F0F0 ) == 0x3030303030303030)
    //  && (( (val + 0x0606060606060606) & 0xF0F0F0F0F0F0F0F0 ) ==
    //  0x3030303030303030);
    (((val & 0xF0F0F0F0F0F0F0F0) | (((val + 0x0606060606060606) & 0xF0F0F0F0F0F0F0F0) >> 4))
        == 0x3333333333333333)
}
/*
#else
// this is more efficient apparently than the scalar code above (fewer instructions)
    #[cfg_attr(not(feature = "no-inline"), inline)]

unsafe fn is_made_of_eight_digits_fast(chars: *const u8) -> bool {
    let val: __m64 = *(chars as *const __m64);
    let base: __m64 = _mm_sub_pi8(val,_mm_set1_pi8(b'0' as i8));
    let basecmp: __m64 = _mm_subs_pu8(base, _mm_set1_pi8(9));
    _mm_cvtm64_si64(basecmp) == 0
}
 */
pub enum Number {
    F64(f64),
    I64(i64),
}

#[cfg_attr(not(feature = "no-inline"), inline)]
fn parse_eight_digits_unrolled(chars: &[u8]) -> i32 {
    unsafe {
        // this actually computes *16* values so we are being wasteful.
        let ascii0: __m128i = _mm_set1_epi8(b'0' as i8);
        let mul_1_10: __m128i =
            _mm_setr_epi8(10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1);
        let mul_1_100: __m128i = _mm_setr_epi16(100, 1, 100, 1, 100, 1, 100, 1);
        let mul_1_10000: __m128i = _mm_setr_epi16(10000, 1, 10000, 1, 10000, 1, 10000, 1);
        let input: __m128i = _mm_sub_epi8(
            _mm_loadu_si128(chars[0..16].as_ptr() as *const __m128i),
            ascii0,
        );
        let t1: __m128i = _mm_maddubs_epi16(input, mul_1_10);
        let t2: __m128i = _mm_madd_epi16(t1, mul_1_100);
        let t3: __m128i = _mm_packus_epi32(t2, t2);
        let t4: __m128i = _mm_madd_epi16(t3, mul_1_10000);
        _mm_cvtsi128_si32(t4) // only captures the sum of the first 8 digits, drop the rest
    }
}

impl<'de> Deserializer<'de> {
    /// called by parse_number when we know that the output is a float,
    /// but where there might be some integer overflow. The trick here is to
    /// parse using floats from the start.
    /// Do not call this function directly as it skips some of the checks from
    /// parse_number
    ///
    /// This function will almost never be called!!!
    ///
    /// Note: a redesign could avoid this function entirely.
    ///
    #[inline(never)]
    fn parse_float(&self, mut p: &[u8], found_minus: bool) -> Result<Number> {
        let mut negative: bool = false;
        if found_minus {
            p = &p[1..];
            negative = true;
        }
        let mut i: f64;
        if p[0] == b'0' {
            // 0 cannot be followed by an integer
            p = &p[1..];
            i = 0.0;
        } else {
            let mut digit: u8 = p[0] - b'0';
            i = digit as f64;
            p = &p[1..];
            while is_integer(p[0]) {
                digit = p[0] - b'0';
                i = 10.0 * i + digit as f64;
                p = &p[1..];
            }
        }
        if p[0] == b'.' {
            let mut fraction: u64 = 0;
            let mut fractionalweight: u64;
            p = &p[1..];
            //let mut fractionalweight: f64 = 1.0;
            if is_integer(p[0]) {
                let digit: u8 = p[0] - b'0';
                p = &p[1..];
                fractionalweight = 10;
                fraction += digit as u64;
            //i = i + digit as f64 * fractionalweight;
            } else {
                return Err(self.error(ErrorType::Parser));
            }
            while is_integer(p[0]) {
                let digit: u8 = p[0] - b'0';
                p = &p[1..];
                fractionalweight *= 10;
                fraction *= 10;
                fraction += digit as u64;
                //                dbg!(fraction);
                //dbg!(fractionalweight);
            }
            i += fraction as f64 / fractionalweight as f64;
            //dbg!(i);
        }
        if (p[0] == b'e') || (p[0] == b'E') {
            p = &p[1..];
            let mut negexp: bool = false;
            if p[0] == b'-' {
                negexp = true;
                p = &p[1..];
            } else if p[0] == b'+' {
                p = &p[1..];
            }
            if !is_integer(p[0]) {
                return Err(self.error(ErrorType::Parser));
            }
            let mut digit: u8 = p[0] - b'0';
            let mut expnumber: i64 = digit as i64; // exponential part
            p = &p[1..];
            if is_integer(p[0]) {
                digit = p[0] - b'0';
                expnumber = 10 * expnumber + digit as i64;
                p = &p[1..];
            }
            if is_integer(p[0]) {
                digit = p[0] - b'0';
                expnumber = 10 * expnumber + digit as i64;
                p = &p[1..];
            }
            if is_integer(p[0]) {
                digit = p[0] - b'0';
                expnumber = 10 * expnumber + digit as i64;
                p = &p[1..];
            }
            if is_integer(p[0]) {
                // we refuse to parse this
                return Err(self.error(ErrorType::Parser));
            }
            let exponent: i32 = if negexp {
                -expnumber as i32
            } else {
                expnumber as i32
            };
            if (exponent > 308) || (exponent < -308) {
                // we refuse to parse this
                return Err(self.error(ErrorType::Parser));
            }
            i *= POWER_OF_TEN[(308 + exponent) as usize];
        }
        if is_not_structural_or_whitespace(p[0]) != 0 {
            return Err(self.error(ErrorType::Parser));
        }

        if is_structural_or_whitespace(p[0]) != 0 {
            Ok(Number::F64(if negative { -i } else { i }))
        } else {
            Err(self.error(ErrorType::Parser))
        }
    }

    /*
    // called by parse_number when we know that the output is an integer,
    // but where there might be some integer overflow.
    // we want to catch overflows!
    // Do not call this function directly as it skips some of the checks from
    // parse_number
    //
    // This function will almost never be called!!!
    //
    static never_inline bool parse_large_integer(const uint8_t *const buf,
                                                 ParsedJson &pj,
                                                 const uint32_t offset,
                                                 bool found_minus) {
      const char *p = reinterpret_cast<const char *>(buf + offset);

      bool negative = false;
      if (found_minus) {
        ++p;
        negative = true;
      }
      uint64_t i;
      if (*p == '0') { // 0 cannot be followed by an integer
        ++p;
        i = 0;
      } else {
        unsigned char digit = *p - '0';
        i = digit;
        p++;
        // the is_made_of_eight_digits_fast routine is unlikely to help here because
        // we rarely see large integer parts like 123456789
        while (is_integer(*p)) {
          digit = *p - '0';
          if (mul_overflow(i, 10, &i)) {
    #ifdef JSON_TEST_NUMBERS // for unit testing
            foundInvalidNumber(buf + offset);
    #endif
            return false; // overflow
          }
          if (add_overflow(i, digit, &i)) {
    #ifdef JSON_TEST_NUMBERS // for unit testing
            foundInvalidNumber(buf + offset);
    #endif
            return false; // overflow
          }
          ++p;
        }
      }
      if (negative) {
        if (i > 0x8000000000000000) {
    // overflows!
    #ifdef JSON_TEST_NUMBERS // for unit testing
          foundInvalidNumber(buf + offset);
    #endif
          return false; // overflow
        }
      } else {
        if (i >= 0x8000000000000000) {
    // overflows!
    #ifdef JSON_TEST_NUMBERS // for unit testing
          foundInvalidNumber(buf + offset);
    #endif
          return false; // overflow
        }
      }
      int64_t signed_answer = negative ? -static_cast<int64_t>(i) : static_cast<int64_t>(i);
      pj.write_tape_s64(signed_answer);
    #ifdef JSON_TEST_NUMBERS // for unit testing
      foundInteger(signed_answer, buf + offset);
    #endif
      return is_structural_or_whitespace(*p);
    }

    */

    // parse the number at buf + offset
    // define JSON_TEST_NUMBERS for unit testing
    //#[inline(always)]
    #[cfg_attr(not(feature = "no-inline"), inline(always))]
    pub fn parse_number_int(&self, mut buf: &[u8], negative: bool) -> Result<Number> {
        if negative {
            buf = &buf[1..];
            /*
            // We don't need that as the next batch checks:
            // if it's 0 (if branch)
            // or if it's an integer (else branch)
            if !is_integer(p[0]) {
                // a negative sign must be followed by an integer
                return Err(self.error(ErrorType::InvalidNumber));
            }
            */
        }
        //let startdigits: *const u8 = p;
        let mut digitcount = 0;
        let mut i: i64;
        if buf[0] == b'0' {
            // 0 cannot be followed by an integer
            digitcount += 1;
            if is_not_structural_or_whitespace_or_exponent_or_decimal(unsafe {
                *buf.get_unchecked(digitcount)
            }) {
                return Err(self.error(ErrorType::InvalidNumber));
            }
            i = 0;
        } else {
            if !is_integer(buf[0]) {
                // must start with an integer
                return Err(self.error(ErrorType::InvalidNumber));
            }
            let mut digit: u8 = buf[0] - b'0';
            i = digit as i64;
            digitcount += 1;
            // the is_made_of_eight_digits_fast routine is unlikely to help here because
            // we rarely see large integer parts like 123456789
            while is_integer(unsafe { *buf.get_unchecked(digitcount) }) {
                digit = unsafe { *buf.get_unchecked(digitcount) } - b'0';
                i = 10 * i + digit as i64; // might overflow
                digitcount += 1;
            }
        }

        let mut exponent: i64 = 0;
        if b'.' == unsafe { *buf.get_unchecked(digitcount) } {
            digitcount += 1;
            let firstafterperiod = digitcount;
            if is_integer(unsafe { *buf.get_unchecked(digitcount) }) {
                let digit: u8 = unsafe { *buf.get_unchecked(digitcount) } - b'0';
                digitcount += 1;
                i = i * 10 + digit as i64;
            } else {
                return Err(self.error(ErrorType::InvalidNumber));
            }
            // this helps if we have lots of decimals!
            // this turns out to be frequent enough.

            #[cfg(feature = "swar-number-parsing")]
            {
                if buf.len() - digitcount >= 16 && is_made_of_eight_digits_fast(&buf[digitcount..])
                {
                    i = i * 100_000_000 + parse_eight_digits_unrolled(&buf[digitcount..]) as i64;
                    digitcount += 8;
                    // exponent -= 8;
                }
            }

            while is_integer(unsafe { *buf.get_unchecked(digitcount) }) {
                let digit: u8 = unsafe { *buf.get_unchecked(digitcount) } - b'0';
                digitcount += 1;
                i = i * 10 + digit as i64; // in rare cases, this will overflow, but that's ok because we have parse_highprecision_float later.
            }
            exponent = firstafterperiod as i64 - digitcount as i64;
        }
        let mut expnumber: i64 = 0; // exponential part
        let c = unsafe { *buf.get_unchecked(digitcount) };
        if (b'e' == c) || (b'E' == c) {
            digitcount += 1;
            let mut negexp: bool = false;
            if b'-' == unsafe { *buf.get_unchecked(digitcount) } {
                negexp = true;
                digitcount += 1;
            } else if b'+' == unsafe { *buf.get_unchecked(digitcount) } {
                digitcount += 1;
            }
            if !is_integer(unsafe { *buf.get_unchecked(digitcount) }) {
                return Err(self.error(ErrorType::InvalidNumber));
            }
            let mut digit: u8 = unsafe { *buf.get_unchecked(digitcount) } - b'0';
            expnumber = digit as i64;
            digitcount += 1;
            let d = unsafe { *buf.get_unchecked(digitcount) };
            if is_integer(d) {
                digit = d - b'0';
                expnumber = 10 * expnumber + digit as i64;
                digitcount += 1;
            }
            let d = unsafe { *buf.get_unchecked(digitcount) };
            if is_integer(d) {
                digit = d - b'0';
                expnumber = 10 * expnumber + digit as i64;
                digitcount += 1;
            }
            if is_integer(unsafe { *buf.get_unchecked(digitcount) }) {
                // we refuse to parse this
                return Err(self.error(ErrorType::InvalidNumber));
            }
            exponent += if negexp { -expnumber } else { expnumber };
        }
        i = if negative { -i } else { i };
        let v = if (exponent != 0) || (expnumber != 0) {
            if unlikely!(digitcount >= 19) {
                // this is uncommon!!!
                // this is almost never going to get called!!!
                // we start anew, going slowly!!!
                return self.parse_float(buf, negative);
            }
            ///////////
            // We want 0.1e1 to be a float.
            //////////
            if i == 0 {
                Number::F64(0.0)
            } else {
                if (exponent > 308) || (exponent < -308) {
                    // we refuse to parse this
                    return Err(self.error(ErrorType::InvalidExponent));
                }
                let mut d: f64 = i as f64;
                d *= POWER_OF_TEN[(308 + exponent) as usize];
                // d = negative ? -d : d;
                Number::F64(d)
            }
        } else {
            /* TODO: implement this
            if unlikely!(digitcount >= 18) {
                // this is uncommon!!!
                return parse_large_integer(buf, pj, offset, found_minus);
            }
             */
            Number::I64(i)
        };
        if is_structural_or_whitespace(unsafe { *buf.get_unchecked(digitcount) }) != 0 {
            Ok(v)
        } else {
            Err(self.error(ErrorType::InvalidNumber))
        }
    }
}
