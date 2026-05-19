/// Remaps a hardware key position to a logical profile position based on rotation.
/// rows and cols are the original (unrotated) device dimensions.
pub fn hw_to_logical(hw: u8, rows: u8, cols: u8, rotation: u16) -> u8 {
    let r = hw / cols;
    let c = hw % cols;
    match rotation {
        90 => c * rows + (rows - 1 - r),
        180 => rows * cols - 1 - hw,
        270 => (cols - 1 - c) * rows + r,
        _ => hw,
    }
}

/// Remaps a logical profile position to a hardware key position based on rotation.
/// rows and cols are the original (unrotated) device dimensions.
/// For 90°/270° the visual grid has `cols` rows and `rows` cols, so:
///   vr = logical / rows,  vc = logical % rows
pub fn logical_to_hw(logical: u8, rows: u8, cols: u8, rotation: u16) -> u8 {
    let vr = logical / rows;
    let vc = logical % rows;
    match rotation {
        90 => (rows - 1 - vc) * cols + vr,
        180 => rows * cols - 1 - logical,
        270 => vc * cols + (cols - 1 - vr),
        _ => logical,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 2x3 device (R=2, C=3), 90° CW: visual grid is 3 rows x 2 cols
    // Expected visual layout vs hardware positions:
    //   [3][0]
    //   [4][1]
    //   [5][2]
    #[test]
    fn test_90_hw_to_logical() {
        assert_eq!(hw_to_logical(3, 2, 3, 90), 0);
        assert_eq!(hw_to_logical(0, 2, 3, 90), 1);
        assert_eq!(hw_to_logical(4, 2, 3, 90), 2);
        assert_eq!(hw_to_logical(1, 2, 3, 90), 3);
        assert_eq!(hw_to_logical(5, 2, 3, 90), 4);
        assert_eq!(hw_to_logical(2, 2, 3, 90), 5);
    }

    #[test]
    fn test_90_logical_to_hw() {
        assert_eq!(logical_to_hw(0, 2, 3, 90), 3);
        assert_eq!(logical_to_hw(1, 2, 3, 90), 0);
        assert_eq!(logical_to_hw(2, 2, 3, 90), 4);
        assert_eq!(logical_to_hw(3, 2, 3, 90), 1);
        assert_eq!(logical_to_hw(4, 2, 3, 90), 5);
        assert_eq!(logical_to_hw(5, 2, 3, 90), 2);
    }

    // 180°: simple reversal
    #[test]
    fn test_180() {
        assert_eq!(hw_to_logical(0, 2, 3, 180), 5);
        assert_eq!(hw_to_logical(5, 2, 3, 180), 0);
        assert_eq!(logical_to_hw(0, 2, 3, 180), 5);
        assert_eq!(logical_to_hw(5, 2, 3, 180), 0);
    }

    // 270° CW: visual grid is 3 rows x 2 cols
    // Expected visual layout vs hardware positions:
    //   [2][5]
    //   [1][4]
    //   [0][3]
    #[test]
    fn test_270_hw_to_logical() {
        assert_eq!(hw_to_logical(2, 2, 3, 270), 0);
        assert_eq!(hw_to_logical(5, 2, 3, 270), 1);
        assert_eq!(hw_to_logical(1, 2, 3, 270), 2);
        assert_eq!(hw_to_logical(4, 2, 3, 270), 3);
        assert_eq!(hw_to_logical(0, 2, 3, 270), 4);
        assert_eq!(hw_to_logical(3, 2, 3, 270), 5);
    }

    #[test]
    fn test_270_logical_to_hw() {
        assert_eq!(logical_to_hw(0, 2, 3, 270), 2);
        assert_eq!(logical_to_hw(1, 2, 3, 270), 5);
        assert_eq!(logical_to_hw(2, 2, 3, 270), 1);
        assert_eq!(logical_to_hw(3, 2, 3, 270), 4);
        assert_eq!(logical_to_hw(4, 2, 3, 270), 0);
        assert_eq!(logical_to_hw(5, 2, 3, 270), 3);
    }

    #[test]
    fn test_0_identity() {
        for i in 0..6u8 {
            assert_eq!(hw_to_logical(i, 2, 3, 0), i);
            assert_eq!(logical_to_hw(i, 2, 3, 0), i);
        }
    }
}
