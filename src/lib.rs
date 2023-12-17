mod CSV {
    #[derive(Debug)]
    pub struct HeaderCSV {
        pub header: Vec<String>,
        pub data: Vec<Vec<String>>,
    }
    #[derive(Debug)]
    pub struct HeaderlessCSV {
        pub data: Vec<Vec<String>>,
    }
    impl TryFrom<String> for HeaderCSV {
        type Error = CSVError;
        fn try_from(value: String) -> Result<HeaderCSV, CSVError> {
            let line_data = value.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line).map_err(|_| CSVError::InvalidFormatError)?)
            }
            Ok(Self {
                header: new_vec[0]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<String>>(),
                data: new_vec[1..]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<Vec<String>>>(),
            })
        }
    }
    impl TryFrom<&str> for HeaderCSV {
        type Error = CSVError;
        fn try_from(value: &str) -> Result<Self, CSVError> {
            let line_data = value.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line).map_err(|_| CSVError::InvalidFormatError)?)
            }
            Ok(Self {
                header: new_vec[0]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<String>>(),
                data: new_vec[1..]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<Vec<String>>>(),
            })
        }
    }
    impl HeaderCSV {
        pub fn from_file(path: &str) -> Result<Self, CSVError> {
            let str_data = std::fs::read_to_string(path).map_err(|_| CSVError::FileNotFound)?;
            let line_data = str_data.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line)?)
            }
            Ok(Self {
                header: new_vec[0]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<String>>(),
                data: new_vec[1..]
                    .iter()
                    .map(|item| item.to_owned())
                    .collect::<Vec<Vec<String>>>(),
            })
        }
    }
    impl TryFrom<String> for HeaderlessCSV {
        type Error = CSVError;
        fn try_from(value: String) -> Result<Self, CSVError> {
            let line_data = value.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line).map_err(|_| CSVError::InvalidFormatError)?)
            }
            Ok(Self { data: new_vec })
        }
    }
    impl TryFrom<&str> for HeaderlessCSV {
        type Error = CSVError;
        fn try_from(value: &str) -> Result<Self, CSVError> {
            let line_data = value.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line).map_err(|_| CSVError::InvalidFormatError)?)
            }
            Ok(Self { data: new_vec })
        }
    }
    impl From<HeaderCSV> for HeaderlessCSV {
        fn from(value: HeaderCSV) -> Self {
            HeaderlessCSV { data: value.data }
        }
    }
    impl HeaderlessCSV {
        pub fn from_file(path: &str) -> Result<Self, CSVError> {
            let str_data = std::fs::read_to_string(path).map_err(|_| CSVError::FileNotFound)?;
            let line_data = str_data.lines().collect::<Vec<&str>>();
            let mut new_vec: Vec<Vec<String>> = Vec::new();
            for line in line_data {
                new_vec.push(split_on_commas(line)?)
            }
            Ok(Self { data: new_vec })
        }
        pub fn to_header_csv(self, headers: Vec<String>) -> HeaderCSV {
            HeaderCSV {
                header: headers,
                data: self.data,
            }
        }
    }
    fn split_on_commas(line: &str) -> Result<Vec<String>, CSVError> {
        let mut end_vec: Vec<String> = Vec::new();
        let mut is_quoted = false;
        let mut is_apostraphied = false;
        let mut start_index = 0;
        let mut was_quoted_a = false;
        let mut was_quoted_b = false;
        for (index, ch) in line.char_indices() {
            was_quoted_b = was_quoted_a;
            was_quoted_a = is_quoted || is_apostraphied;
            if ch == '\"' && !is_apostraphied {
                is_quoted = !is_quoted;
            } else if ch == '\'' && !is_quoted {
                is_apostraphied = !is_apostraphied;
            } else if ch == ',' && !(is_quoted || is_apostraphied) {
                // end_vec.push(line[start_index..(index)].to_owned());
                //     start_index = index + 2;
                if was_quoted_b {
                    end_vec.push(line[(start_index + 1)..(index - 1)].to_owned());
                    start_index = index + 1;
                } else {
                    end_vec.push(line[start_index..(index)].to_owned());
                    start_index = index + 1;
                }
            }
            // todo!("Fix split_on_c    ommas")
        }
        if was_quoted_b {
            end_vec.push(line[(start_index + 1)..(line.len() - 1)].to_owned());
        } else {
            end_vec.push(line[start_index..(line.len())].to_owned());
        }
        Ok(end_vec)
    }
    #[derive(Debug)]
    pub enum CSVError {
        InvalidFormatError,
        FileNotFound,
    }
    #[cfg(test)]
    mod csv_tests {
        use super::*;
        #[test]
        fn header_headers() {
            let result = HeaderCSV::from_file("src/csvs/headered.csv")
                .unwrap()
                .header;
            assert_eq!(
                result,
                vec!["Date", "Open", "High", "Low", "Close", "Volume"]
            );
        }
        #[test]
        fn header_data() {
            let result = HeaderCSV::from_file("src/csvs/headered.csv").unwrap().data;
            assert_eq!(
                result[0],
                vec![
                    "11/14/2023",
                    "371.01",
                    "371.95",
                    "367.35",
                    "370.27",
                    "27,683,859"
                ]
            );
        }
        #[test]
        fn headerless_data() {
            let result = HeaderlessCSV::from_file("src/csvs/headerless.csv")
                .unwrap()
                .data;
            assert_eq!(
                result[0],
                vec![
                    "11/14/2023",
                    "371.01",
                    "371.95",
                    "367.35",
                    "370.27",
                    "27,683,859"
                ]
            );
        }
        #[test]
        fn split_on_commas_test() {
            let x = r#"11/14/2023,"371.01","371.95","367.35","370.27","27,683,859""#;
            assert_eq!(
                vec![
                    "11/14/2023",
                    "371.01",
                    "371.95",
                    "367.35",
                    "370.27",
                    "27,683,859"
                ],
                split_on_commas(x).unwrap()
            )
        }
    }
}
