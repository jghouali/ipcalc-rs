use std::fmt::Display;
#[derive(Debug, Clone, Copy)]
pub enum IP {
    IPV4 { prefix: u32, mask: u8 },
    IPV6 { prefix: u128, mask: u8 },
}
#[derive(Debug, Clone, Copy)]
pub enum HostCount {
    V4(u32),
    V6(u128),
}
impl Display for HostCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HostCount::V4(count) => count.to_string(),
                HostCount::V6(count) => count.to_string(),
            }
        )
    }
}
impl IP {
    pub fn cidr_to_mask_ipv4(cidr: u8) -> u32 {
        let mask = u32::MAX << (32 - cidr);
        //println!("cidrToMaskIPv4: cidr = {} -> mask = {}", cidr, mask);
        mask
    }
    pub fn cidr_to_mask_ipv6(cidr: u8) -> u128 {
        let mask = u128::MAX << (128 - cidr);
        //println!("cidrToMaskIPv6: cidr = {} -> mask = {}", cidr, mask);
        mask
    }
    pub fn network(&self) -> IP {
        match self {
            IP::IPV4 { prefix, mask } => IP::IPV4 {
                prefix: prefix & Self::cidr_to_mask_ipv4(*mask),
                mask: *mask,
            },
            IP::IPV6 { prefix, mask } => IP::IPV6 {
                prefix: prefix & Self::cidr_to_mask_ipv6(*mask),
                mask: *mask,
            },
        }
    }
    pub fn broadcast(&self) -> IP {
        match self {
            IP::IPV4 { prefix, mask } => IP::IPV4 {
                prefix: prefix | (!Self::cidr_to_mask_ipv4(*mask)),
                mask: *mask,
            },
            IP::IPV6 { prefix, mask } => IP::IPV6 {
                prefix: prefix | (!Self::cidr_to_mask_ipv6(*mask)),
                mask: *mask,
            },
        }
    }

    pub fn mask(&self) -> IP {
        match self {
            IP::IPV4 { prefix: _, mask } => IP::IPV4 {
                prefix: Self::cidr_to_mask_ipv4(*mask),
                mask: *mask,
            },
            IP::IPV6 { prefix: _, mask } => IP::IPV6 {
                prefix: Self::cidr_to_mask_ipv6(*mask),
                mask: *mask,
            },
        }
    }

    pub fn hostmin(&self) -> IP {
        match self {
            IP::IPV4 { prefix, mask } => IP::IPV4 {
                prefix: (prefix & Self::cidr_to_mask_ipv4(*mask)) + 1,
                mask: *mask,
            },
            IP::IPV6 { prefix, mask } => IP::IPV6 {
                prefix: (prefix & Self::cidr_to_mask_ipv6(*mask)) + 1,
                mask: *mask,
            },
        }
    }

    pub fn hostmax(&self) -> IP {
        match self {
            IP::IPV4 { prefix, mask } => IP::IPV4 {
                prefix: (prefix | (!Self::cidr_to_mask_ipv4(*mask))) - 1,
                mask: *mask,
            },
            IP::IPV6 { prefix, mask } => IP::IPV6 {
                prefix: (prefix | (!Self::cidr_to_mask_ipv6(*mask))) - 1,
                mask: *mask,
            },
        }
    }

    pub fn count(&self) -> Result<HostCount, String> {
        match self {
            IP::IPV4 { prefix, mask } => Ok(HostCount::V4(
                (prefix | (!Self::cidr_to_mask_ipv4(*mask)))
                    - (prefix & Self::cidr_to_mask_ipv4(*mask))
                    - 1,
            )),
            IP::IPV6 { prefix, mask } => Ok(HostCount::V6(
                (prefix | (!Self::cidr_to_mask_ipv6(*mask)))
                    - (prefix & Self::cidr_to_mask_ipv6(*mask))
                    - 1,
            )),
        }
    }

    pub fn from_str(s: &str) -> Result<IP, String> {
        let slices = s.split('/').collect::<Vec<&str>>();
        let ip_str: &str;
        let cidr: Option<u8>;
        match slices.as_slice() {
            [ip_tmp] => (ip_str, cidr) = (ip_tmp, None),
            [ip_tmp, cidr_tmp] => {
                cidr = Some(cidr_tmp.parse::<u8>().map_err(|e| e.to_string())?);
                ip_str = ip_tmp
            }
            _ => return Err(format!("Error in the given cidr: {}", s)),
        }
        match (ip_str.contains('.'), ip_str.contains(':')) {
            (true, false) => {
                let ip_vec = ip_str.split('.').collect::<Vec<&str>>();
                match ip_vec.as_slice() {
                    [a, b, c, d] => {
                        let octet_a = a.parse::<u8>().map_err(|e| e.to_string())?;
                        let octet_b = b.parse::<u8>().map_err(|e| e.to_string())?;
                        let octet_c = c.parse::<u8>().map_err(|e| e.to_string())?;
                        let octet_d = d.parse::<u8>().map_err(|e| e.to_string())?;
                        let prefix = ((octet_a as u32) << 24)
                            | ((octet_b as u32) << 16)
                            | ((octet_c as u32) << 8)
                            | (octet_d as u32);
                        if cidr.is_some() && cidr.unwrap() > 32 {
                            Err(format!("cidr>32: {}", s))?;
                        }
                        Ok(IP::IPV4 {
                            prefix,
                            mask: cidr.unwrap_or(32),
                        })
                    }
                    _ => Err("The IPv4 must have 4 bytes".to_string())?,
                }
            }
            (false, true) => {
                let ip_str_to_format = ip_str.replace("::", ":ZZ:");
                let ip_vec = ip_str_to_format.split(':').collect::<Vec<&str>>();

                let count_to_put = 8 - ip_vec.len();

                let mut ip_vec_tmp = ip_vec.clone();

                for i in 0..ip_vec_tmp.len() {
                    if ip_vec_tmp[i] == "ZZ" {
                        ip_vec_tmp.remove(i);
                        for _ in 0..count_to_put + 1 {
                            ip_vec_tmp.insert(i, "0000" as &str);
                        }
                    }
                }

                match ip_vec_tmp.as_slice() {
                    [a, b, c, d, e, f, g, h] => {
                        let octet_a = str_to_hex(a)?;
                        let octet_b = str_to_hex(b)?;
                        let octet_c = str_to_hex(c)?;
                        let octet_d = str_to_hex(d)?;
                        let octet_e = str_to_hex(e)?;
                        let octet_f = str_to_hex(f)?;
                        let octet_g = str_to_hex(g)?;
                        let octet_h = str_to_hex(h)?;
                        let prefix = ((octet_a as u128) << 112)
                            | ((octet_b as u128) << 96)
                            | ((octet_c as u128) << 80)
                            | ((octet_d as u128) << 64)
                            | ((octet_e as u128) << 48)
                            | ((octet_f as u128) << 32)
                            | ((octet_g as u128) << 16)
                            | octet_h as u128;
                        if cidr.is_some() && cidr.unwrap() > 128 {
                            Err(format!("cidr>128: {}", s))?;
                        }
                        Ok(IP::IPV6 {
                            prefix,
                            mask: cidr.unwrap_or(128),
                        })
                    }
                    _ => Err("The IPV6 must have 8 bytes".to_string())?,
                }
            }
            (true, true) => Err("The IPv4 and IPv6 cannot be mixed".to_string())?,
            (false, false) => Err("The IPv4 and IPv6 cannot be mixed".to_string())?,
        }
    }
}
fn str_to_hex(s: &str) -> Result<u16, String> {
    let mut result = 0;
    for c in s.chars() {
        match c {
            '0'..='9' => {
                result = result * 16 + (c as u16 - '0' as u16);
            }
            'a'..='f' => {
                result = result * 16 + (c as u16 - 'a' as u16 + 10);
            }
            _ => return Err(format!("Invalid character: {}", c)),
        }
    }
    Ok(result)
}
