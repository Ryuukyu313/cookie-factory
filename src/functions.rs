use gen::GenError;

pub trait SerializeFn<I>: Fn(I) -> Result<I, GenError> {}

impl<I, F:  Fn(I) ->Result<I, GenError>> SerializeFn<I> for F {}


pub fn slice<'a, S: 'a + AsRef<[u8]>>(data: S) -> impl SerializeFn<&'a mut [u8]> {
    let len = data.as_ref().len();

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            (&mut out[..len]).copy_from_slice(data.as_ref());
            Ok(&mut out[len..])
        }
    }
}

pub fn string<'a, S: 'a+AsRef<str>>(data: S) -> impl SerializeFn<&'a mut [u8]> {

    let len = data.as_ref().len();
    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            (&mut out[..len]).copy_from_slice(data.as_ref().as_bytes());
            Ok(&mut out[len..])
        }
    }
}

pub fn skip<'a>(len: usize) -> impl SerializeFn<&'a mut [u8]> {

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            Ok(&mut out[len..])
        }
    }
}

pub fn position<'a, F>(f: F) -> impl Fn(&'a mut [u8]) -> Result<(&'a mut [u8], &'a mut [u8]), GenError>
  where F: SerializeFn<&'a mut [u8]> {

    move |out: &'a mut [u8]| {
        unsafe {
            let ptr = out.as_mut_ptr();
            let out = f(out)?;
            let len = out.as_ptr() as usize - ptr as usize;

            Ok((std::slice::from_raw_parts_mut(ptr, len), out))
        }
    }
}

fn pair<F, G, I>(first: F, second: G) -> impl SerializeFn<I>
where F: SerializeFn<I>,
      G: SerializeFn<I> {

  move |out: I| {
    let out = first(out)?;
    second(out)
  }
}

fn cond<F, G, I>(condition: bool, f: F) -> impl SerializeFn<I>
where F: SerializeFn<I>, {

  move |out: I| {
    if condition {
      f(out)
    } else {
      Ok(out)
    }
  }
}


pub fn _all<'a, 'b, G, I, It: Iterator<Item=G>, Arg: 'a+Clone+IntoIterator<Item=G, IntoIter=It>>(values: Arg) -> impl SerializeFn<I> + 'a
  where G: SerializeFn<I> + 'b {

  move |mut out: I| {
    let mut it = values.clone().into_iter();

    for v in it {
      out = v(out)?;
    }

    Ok(out)
  }
}

pub fn separated_list<'a, 'b, 'c, F, G, I, It: Iterator<Item=G>, Arg: 'a+Clone+IntoIterator<Item=G, IntoIter=It>>(sep: F, values: Arg) -> impl SerializeFn<I> + 'a
  where F: SerializeFn<I> + 'b + 'a,
        G: SerializeFn<I> + 'c {

  move |mut out: I| {
    let mut it = values.clone().into_iter();
    match it.next() {
      None => return Ok(out),
      Some(first) => {
        out = first(out)?;
      }
    }

    for v in it {
      out = sep(out)?;
      out = v(out)?;
    }

    Ok(out)
  }
}

pub fn be_u8<'a>(i: u8) -> impl SerializeFn<&'a mut [u8]> {
   let len = 1;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = i;
            Ok(&mut out[len..])
        }
    }
}

pub fn be_u16<'a>(i: u16) -> impl SerializeFn<&'a mut [u8]> {
   let len = 2;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = ((i >> 8) & 0xff) as u8;
            out[1] = (i        & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}

pub fn be_u32<'a>(i: u32) -> impl SerializeFn<&'a mut [u8]> {
   let len = 4;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = ((i >> 24) & 0xff) as u8;
            out[1] = ((i >> 16) & 0xff) as u8;
            out[2] = ((i >> 8)  & 0xff) as u8;
            out[3] = (i         & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}

pub fn be_u64<'a>(i: u64) -> impl SerializeFn<&'a mut [u8]> {
   let len = 8;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = ((i >> 56) & 0xff) as u8;
            out[1] = ((i >> 48) & 0xff) as u8;
            out[2] = ((i >> 40) & 0xff) as u8;
            out[3] = ((i >> 32) & 0xff) as u8;
            out[4] = ((i >> 24) & 0xff) as u8;
            out[5] = ((i >> 16) & 0xff) as u8;
            out[6] = ((i >> 8)  & 0xff) as u8;
            out[7] = (i         & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}

pub fn le_u8<'a>(i: u8) -> impl SerializeFn<&'a mut [u8]> {
   let len = 1;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = i;
            Ok(&mut out[len..])
        }
    }
}

pub fn le_u16<'a>(i: u16) -> impl SerializeFn<&'a mut [u8]> {
   let len = 2;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = (i        & 0xff) as u8;
            out[1] = ((i >> 8) & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}

pub fn le_u32<'a>(i: u32) -> impl SerializeFn<&'a mut [u8]> {
   let len = 4;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = (i         & 0xff) as u8;
            out[1] = ((i >> 8)  & 0xff) as u8;
            out[2] = ((i >> 16) & 0xff) as u8;
            out[3] = ((i >> 24) & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}

pub fn le_u64<'a>(i: u64) -> impl SerializeFn<&'a mut [u8]> {
   let len = 8;

    move |out: &'a mut [u8]| {
        if out.len() < len {
            Err(GenError::BufferTooSmall(len))
        } else {
            out[0] = (i         & 0xff) as u8;
            out[1] = ((i >> 8)  & 0xff) as u8;
            out[2] = ((i >> 16) & 0xff) as u8;
            out[3] = ((i >> 24) & 0xff) as u8;
            out[4] = ((i >> 32) & 0xff) as u8;
            out[5] = ((i >> 40) & 0xff) as u8;
            out[6] = ((i >> 48) & 0xff) as u8;
            out[7] = ((i >> 56) & 0xff) as u8;
            Ok(&mut out[len..])
        }
    }
}
///missing combinators:
///or
///empty
///then
///stream
///length_value
///text print
///text upperhex
///text lowerhex
struct Dummy;