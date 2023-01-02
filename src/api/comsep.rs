use std::fmt::Display;

pub trait ComSep<T: Display> {
    fn to_cs_string(self) -> String;
}

impl<T: Display> ComSep<T> for T {
    fn to_cs_string(self) -> String {
        self.to_string()
    }
}

impl<T: Display> ComSep<T> for &T {
    fn to_cs_string(self) -> String {
        self.to_string()
    }
}

impl<T: Display, const N: usize> ComSep<T> for [T; N] {
    fn to_cs_string(self) -> String {
        self.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .to_string()
    }
}

impl<T: Display> ComSep<T> for Vec<T> {
    fn to_cs_string(self) -> String {
        self.iter()
            .map(|a| a.to_string())
            .collect::<Vec<_>>()
            .join(",")
            .to_string()
    }
}
