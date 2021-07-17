use bdaddr::Address;
use super::*;

fn fill<R>(mut this: R, mut buf: &mut [u8]) -> Result<()>
where
    R: io::Read,
{
    let mut total_read = 0;
    while !buf.is_empty() {
        match this.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                total_read += n;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }

    if total_read == 0 {
        Err(Error::NoDataAvailable)
    } else if !buf.is_empty() {
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer").into())
    } else {
        Ok(())
    }
}

impl Pack for () {
    fn pack<W>(&self, _: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        Ok(())
    }
}

impl Unpack for () {
    fn unpack<R>(_: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(())
    }
}

impl Pack for bool {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        (if *self { 1u8 } else { 0u8 }).pack(write)
    }
}

impl Unpack for bool {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(u8::unpack(read)? != 0)
    }
}

impl Pack for u8 {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u8 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let v = <[u8; 1]>::unpack(read)?;
        Ok(Self::from_le_bytes(v))
    }
}

impl Pack for u16 {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u16 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let v = <[u8; 2]>::unpack(read)?;
        Ok(Self::from_le_bytes(v))
    }
}

impl Pack for u32 {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u32 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let v = <[u8; 4]>::unpack(read)?;
        Ok(Self::from_le_bytes(v))
    }
}

impl Pack for u128 {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u128 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let v = <[u8; 16]>::unpack(read)?;
        Ok(Self::from_le_bytes(v))
    }
}

impl<T> Pack for Option<T>
where
    T: Pack,
{
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        if let Some(v) = self {
            v.pack(write)
        } else {
            Ok(())
        }
    }
}

impl<T> Unpack for Option<T>
where
    T: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        match T::unpack(read) {
            Ok(v) => Ok(Some(v)),
            Err(Error::NoDataAvailable) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl<T> Pack for Box<T>
where
    T: Pack,
{
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        T::pack(&*self, write)
    }
}

impl<T> Unpack for Box<T>
where
    T: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(Box::new(T::unpack(read)?))
    }
}

impl<T> Pack for Vec<T>
where
    T: Pack,
{
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        (self.len() as u16).pack(write)?;
        for item in self {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl<T> Unpack for Vec<T>
where
    T: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let len = u16::unpack(read)?;
        (0..len).map(|_| T::unpack(read)).collect()
    }
}

impl Pack for Box<[u8]> {
    fn pack<W>(&self, write: &mut W) -> Result<()> where W: io::Write {
        write.write_all(self)?;
        Ok(())
    }
}

impl Unpack for Box<[u8]> {
    fn unpack<R>(read: &mut R) -> Result<Self> where R: io::Read {
        let mut b = vec![];
        read.read_to_end(&mut b)?;
        Ok(Box::from(&b[..]))
    }
}

impl<const N: usize> Pack for [u8; N] {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        write.write_all(&self[..])?;
        Ok(())
    }
}

impl<const N: usize> Unpack for [u8; N] {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let mut buf = [0; N];
        fill(read, &mut buf)?;
        Ok(buf)
    }
}

impl<P1, P2> Pack for (P1, P2) where P1: Pack, P2: Pack {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where W: io::Write {
        let (p1, p2) = self;
        p1.pack(write)?;
        p2.pack(write)?;
        Ok(())
    }
}

impl<P1, P2> Unpack for (P1, P2) where P1: Unpack, P2: Unpack {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where R: io::Read {
        Ok((
            P1::unpack(read)?,
            P2::unpack(read)?,
        ))
    }
}

impl<P1, P2, P3> Pack for (P1, P2, P3) where P1: Pack, P2: Pack, P3: Pack {
    fn pack<W>(&self, write: &mut W) -> Result<()>
    where W: io::Write {
        let (p1, p2, p3) = self;
        p1.pack(write)?;
        p2.pack(write)?;
        p3.pack(write)?;
        Ok(())
    }
}

impl<P1, P2, P3> Unpack for (P1, P2, P3) where P1: Unpack, P2: Unpack, P3: Unpack {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where R: io::Read {
        Ok((
            P1::unpack(read)?,
            P2::unpack(read)?,
            P3::unpack(read)?,
        ))
    }
}

impl Pack for Address {
    fn pack<W>(&self, write: &mut W) -> Result<()> where W: io::Write {
        <[u8; 6]>::pack(&self.clone().into(), write)
    }
}

impl Unpack for Address {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where R: io::Read {
        <[u8; 6]>::unpack(read).map(From::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit() {
        let mut b = vec![];

        ().pack(&mut b).unwrap();
        assert!(b.is_empty());

        let v = <()>::unpack(&mut &b[..]).unwrap();
        assert_eq!((), v);
    }

    #[test]
    fn test_bool() {
        let tests = [
            (true, &[1]),
            (false, &[0]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = bool::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_u16() {
        let tests = [
            (0x00FF, &[0xFF, 0x00]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = u16::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_u32() {
        let tests = [
            (0x00FFFFFF, &[0xFF, 0xFF, 0xFF, 0x00]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = u32::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_u128() {
        let tests = [
            (0x00FFFFFFFFFFFFFFFFFFFFFFFFFFFFFF, &[0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = u128::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_opt() {
        let tests = [
            (Some(1u8), &[1][..]),
            (None, &[][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <Option<u8>>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_box() {
        let tests = [
            (Box::new(1u8), &[1][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <Box<u8>>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_vec() {
        let tests = [
            (vec![2, 5], &[0x02, 0x00, 0x02, 0x05][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <Vec<u8>>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_box_slice() {
        let tests = [
            (Box::<[u8]>::from(&[2, 5][..]), &[0x02, 0x05][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <Box<[u8]>>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_tuple2() {
        let tests = [
            ((true, 2u8), &[0x01, 0x02][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <(bool, u8)>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_tuple3() {
        let tests = [
            ((true, 2u8, 3u8), &[0x01, 0x02, 0x03][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <(bool, u8, u8)>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_addr() {
        let tests = [
            ("00:11:22:33:44:55".parse::<Address>().unwrap(), &[0x55, 0x44, 0x33, 0x22, 0x11, 0x00][..]),
        ];

        for (test, buf) in tests {
            let mut b = vec![];
            test.pack(&mut b).unwrap();
            assert_eq!(b, buf);

            let v = <Address>::unpack(&mut &b[..]).unwrap();
            assert_eq!(v, test);
        }
    }

    #[test]
    fn test_unexpected_eof() {
        let b = vec![0x01];
        let r = <[u8; 2]>::unpack(&mut &b[..]);
        assert!(matches!(r, Err(Error::Io(io::Error {..} ))))
    }

    #[test]
    fn test_some_io_err() {
        struct R(bool);
        impl io::Read for R {
            fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
                if !self.0 {
                    self.0 = true;
                    Err(io::Error::new(io::ErrorKind::Interrupted, "int"))
                } else {
                    Err(io::Error::new(io::ErrorKind::Other, "other"))
                }
            }
        }

        let mut r = R(false);
        <Option<u8>>::unpack(&mut r).unwrap_err();
    }
}
