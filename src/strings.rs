use io::*;

pub fn format_data_size_en(size: usize) -> String {
    match size {
        size if size >= BYTES_PER_PB => format_data_size_unit(size, BYTES_PER_PB, "PB"),
        size if size >= BYTES_PER_TB => format_data_size_unit(size, BYTES_PER_TB, "TB"),
        size if size >= BYTES_PER_GB => format_data_size_unit(size, BYTES_PER_GB, "GB"),
        size if size >= BYTES_PER_MB => format_data_size_unit(size, BYTES_PER_MB, "MB"),
        size if size >= BYTES_PER_KB => format_data_size_unit(size, BYTES_PER_KB, "kB"),
        size => size.to_string() + " B"
    }
}

fn format_data_size_unit(size: usize, unit: usize, unit_name: &str) -> String {
    if size % unit == 0 {
        format!("{} {}", size / unit, unit_name)
    } else {
        format!("{:.1} {}", (size as f64) / (unit as f64), unit_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_data_size_en() {
        assert_eq!(format_data_size_en(BYTES_PER_PB), "1 PB");
        assert_eq!(format_data_size_en(BYTES_PER_TB), "1 TB");
        assert_eq!(format_data_size_en(BYTES_PER_GB), "1 GB");
        assert_eq!(format_data_size_en(BYTES_PER_MB), "1 MB");
        assert_eq!(format_data_size_en(BYTES_PER_KB), "1 kB");

        assert_eq!(format_data_size_en(1955523111111411156), "1736.9 PB");
        assert_eq!(format_data_size_en(1955511111411156), "1.7 PB");
        assert_eq!(format_data_size_en(1951111411156), "1.8 TB");
        assert_eq!(format_data_size_en(1511411156), "1.4 GB");
        assert_eq!(format_data_size_en(1511156), "1.4 MB");
        assert_eq!(format_data_size_en(151156), "147.6 kB");
        assert_eq!(format_data_size_en(1025), "1.0 kB");
        assert_eq!(format_data_size_en(1023), "1023 B");
    }
}
