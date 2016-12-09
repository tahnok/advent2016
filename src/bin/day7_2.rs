use std::io;
use std::io::Read;
use std::str::FromStr;


fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);
    let sum = IPv7::sum_valid(input.lines().collect::<Vec<&str>>());
    println!("{}", sum);
}

#[derive(Debug)]
pub struct IPv7 {
    hypernets: Vec<String>,
    supernets: Vec<String>,
}

impl IPv7 {
    fn supports_ssl(&self) -> bool {
        let abas = self.abas();
        let babs = self.babs();
        for aba in abas {
            for bab in babs.iter() {
                if aba.0 == bab.1 && aba.1 == bab.0 {
                    return true;
                }
            }
        }
        false
    }

    fn abas(&self) -> Vec<(char, char)> {
        let mut abas = vec![];
        for supernet in self.supernets.iter() {
            let aba = extract(supernet);
            abas.extend(aba);
        }
        abas
    }

    fn babs(&self) -> Vec<(char, char)> {
        let mut babs = vec![];
        for hypernet in self.hypernets.iter() {
            let bab = extract(hypernet);
            babs.extend(bab);
        }
        babs
    }

    fn sum_valid(ips: Vec<&str>) -> u32 {
        let mut sum = 0;
        for ip in ips {
            let parsed: IPv7 = ip.parse().unwrap();
            if parsed.supports_ssl() {
                sum += 1;
            }
        }
        sum
    }
}

impl FromStr for IPv7 {
    type Err = IPv7Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splits = s.split(|c| c == '[' || c == ']');
        let mut hypernets = vec![];
        let mut supernets = vec![];

        for (index, split) in splits.enumerate() {
            if index % 2 == 0 {
                supernets.push(split.to_string());
            } else {
                hypernets.push(split.to_string());
            }
        }

        Ok(IPv7{
            hypernets: hypernets,
            supernets: supernets,
        })
    }
}

#[derive(Debug)]
pub struct IPv7Err {
}

pub fn extract(input: &str) -> Vec<(char, char)> {
    let mut result = vec![];
    let raw = input.bytes().collect::<Vec<u8>>();
    for i in 2..raw.len() {
        if raw[i - 2] != raw[i - 1] && raw[i - 2] == raw[i]  {
            result.push((raw[i] as char , raw[i-1] as char ));
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_parses_ipv7() {
        let _: IPv7 = "abba[mnop]qrst".parse().unwrap();
    }

    #[test]
    fn it_extracts_hypernet_sequences() {
        let ipv7: IPv7 = "onmmhtsykubbpdiqvjm[kbfbiyjyuzmemaomkwa]prqwqocsihfnslooel[hysggeprqecalydywlk]taghiwhgnujsduhnffu[ibpvowghgttfsvt]wcajwcxhcriflxi".parse().unwrap();
        assert_eq!(vec!["kbfbiyjyuzmemaomkwa", "hysggeprqecalydywlk", "ibpvowghgttfsvt"], ipv7.hypernets);
    }

    #[test]
    fn it_extracts_supernet_sequences() {
        let ipv7: IPv7 = "onmmhtsykubbpdiqvjm[kbfbiyjyuzmemaomkwa]prqwqocsihfnslooel[hysggeprqecalydywlk]taghiwhgnujsduhnffu[ibpvowghgttfsvt]wcajwcxhcriflxi".parse().unwrap();
        assert_eq!(vec!["onmmhtsykubbpdiqvjm", "prqwqocsihfnslooel", "taghiwhgnujsduhnffu", "wcajwcxhcriflxi"], ipv7.supernets);
    }

    #[test]
    fn it_extracts_abas() {
        let ipv7: IPv7 = "aba[bab]xyz".parse().unwrap();
        assert_eq!(vec![('a', 'b')], ipv7.abas());
    }

    #[test]
    fn it_extracts_babs() {
        let ipv7: IPv7 = "aba[bab]xyz".parse().unwrap();
        assert_eq!(vec![('b', 'a')], ipv7.babs());
    }

    #[test]
    fn it_extracts() {
        assert_eq!(vec![('z', 'a'), ('z', 'b')], extract("zazbz"));
    }

    #[test]
    fn it_sums_valid() {
        let ips = vec![
            "aba[bab]xyz",
            "xyx[xyx]xyx",
            "aaa[kek]eke",
            "zazbz[bzb]cdb",
        ];
        assert_eq!(3, IPv7::sum_valid(ips));
    }
}
