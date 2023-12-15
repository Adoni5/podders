// automatically generated by the FlatBuffers compiler, do not modify


// @generated

use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod minknow {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod reads_format {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_CONTENT_TYPE: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_CONTENT_TYPE: i16 = 4;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_CONTENT_TYPE: [ContentType; 5] = [
  ContentType::ReadsTable,
  ContentType::SignalTable,
  ContentType::ReadIdIndex,
  ContentType::OtherIndex,
  ContentType::RunInfoTable,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct ContentType(pub i16);
#[allow(non_upper_case_globals)]
impl ContentType {
  pub const ReadsTable: Self = Self(0);
  pub const SignalTable: Self = Self(1);
  pub const ReadIdIndex: Self = Self(2);
  pub const OtherIndex: Self = Self(3);
  pub const RunInfoTable: Self = Self(4);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 4;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::ReadsTable,
    Self::SignalTable,
    Self::ReadIdIndex,
    Self::OtherIndex,
    Self::RunInfoTable,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::ReadsTable => Some("ReadsTable"),
      Self::SignalTable => Some("SignalTable"),
      Self::ReadIdIndex => Some("ReadIdIndex"),
      Self::OtherIndex => Some("OtherIndex"),
      Self::RunInfoTable => Some("RunInfoTable"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for ContentType {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for ContentType {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for ContentType {
    type Output = ContentType;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for ContentType {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for ContentType {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for ContentType {}
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MIN_FORMAT: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
pub const ENUM_MAX_FORMAT: i16 = 0;
#[deprecated(since = "2.0.0", note = "Use associated constants instead. This will no longer be generated in 2021.")]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_FORMAT: [Format; 1] = [
  Format::FeatherV2,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Format(pub i16);
#[allow(non_upper_case_globals)]
impl Format {
  pub const FeatherV2: Self = Self(0);

  pub const ENUM_MIN: i16 = 0;
  pub const ENUM_MAX: i16 = 0;
  pub const ENUM_VALUES: &'static [Self] = &[
    Self::FeatherV2,
  ];
  /// Returns the variant's name or "" if unknown.
  pub fn variant_name(self) -> Option<&'static str> {
    match self {
      Self::FeatherV2 => Some("FeatherV2"),
      _ => None,
    }
  }
}
impl core::fmt::Debug for Format {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    if let Some(name) = self.variant_name() {
      f.write_str(name)
    } else {
      f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
    }
  }
}
impl<'a> flatbuffers::Follow<'a> for Format {
  type Inner = Self;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    let b = flatbuffers::read_scalar_at::<i16>(buf, loc);
    Self(b)
  }
}

impl flatbuffers::Push for Format {
    type Output = Format;
    #[inline]
    unsafe fn push(&self, dst: &mut [u8], _written_len: usize) {
        flatbuffers::emplace_scalar::<i16>(dst, self.0);
    }
}

impl flatbuffers::EndianScalar for Format {
  type Scalar = i16;
  #[inline]
  fn to_little_endian(self) -> i16 {
    self.0.to_le()
  }
  #[inline]
  #[allow(clippy::wrong_self_convention)]
  fn from_little_endian(v: i16) -> Self {
    let b = i16::from_le(v);
    Self(b)
  }
}

impl<'a> flatbuffers::Verifiable for Format {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    i16::run_verifier(v, pos)
  }
}

impl flatbuffers::SimpleToVerifyInSlice for Format {}
pub enum EmbeddedFileOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct EmbeddedFile<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for EmbeddedFile<'a> {
  type Inner = EmbeddedFile<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> EmbeddedFile<'a> {
  pub const VT_OFFSET: flatbuffers::VOffsetT = 4;
  pub const VT_LENGTH: flatbuffers::VOffsetT = 6;
  pub const VT_FORMAT: flatbuffers::VOffsetT = 8;
  pub const VT_CONTENT_TYPE: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    EmbeddedFile { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args EmbeddedFileArgs
  ) -> flatbuffers::WIPOffset<EmbeddedFile<'bldr>> {
    let mut builder = EmbeddedFileBuilder::new(_fbb);
    builder.add_length(args.length);
    builder.add_offset(args.offset);
    builder.add_content_type(args.content_type);
    builder.add_format(args.format);
    builder.finish()
  }


  #[inline]
  pub fn offset(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(EmbeddedFile::VT_OFFSET, Some(0)).unwrap()}
  }
  #[inline]
  pub fn length(&self) -> i64 {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<i64>(EmbeddedFile::VT_LENGTH, Some(0)).unwrap()}
  }
  #[inline]
  pub fn format(&self) -> Format {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<Format>(EmbeddedFile::VT_FORMAT, Some(Format::FeatherV2)).unwrap()}
  }
  #[inline]
  pub fn content_type(&self) -> ContentType {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<ContentType>(EmbeddedFile::VT_CONTENT_TYPE, Some(ContentType::ReadsTable)).unwrap()}
  }
}

impl flatbuffers::Verifiable for EmbeddedFile<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<i64>("offset", Self::VT_OFFSET, false)?
     .visit_field::<i64>("length", Self::VT_LENGTH, false)?
     .visit_field::<Format>("format", Self::VT_FORMAT, false)?
     .visit_field::<ContentType>("content_type", Self::VT_CONTENT_TYPE, false)?
     .finish();
    Ok(())
  }
}
pub struct EmbeddedFileArgs {
    pub offset: i64,
    pub length: i64,
    pub format: Format,
    pub content_type: ContentType,
}
impl<'a> Default for EmbeddedFileArgs {
  #[inline]
  fn default() -> Self {
    EmbeddedFileArgs {
      offset: 0,
      length: 0,
      format: Format::FeatherV2,
      content_type: ContentType::ReadsTable,
    }
  }
}

pub struct EmbeddedFileBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> EmbeddedFileBuilder<'a, 'b> {
  #[inline]
  pub fn add_offset(&mut self, offset: i64) {
    self.fbb_.push_slot::<i64>(EmbeddedFile::VT_OFFSET, offset, 0);
  }
  #[inline]
  pub fn add_length(&mut self, length: i64) {
    self.fbb_.push_slot::<i64>(EmbeddedFile::VT_LENGTH, length, 0);
  }
  #[inline]
  pub fn add_format(&mut self, format: Format) {
    self.fbb_.push_slot::<Format>(EmbeddedFile::VT_FORMAT, format, Format::FeatherV2);
  }
  #[inline]
  pub fn add_content_type(&mut self, content_type: ContentType) {
    self.fbb_.push_slot::<ContentType>(EmbeddedFile::VT_CONTENT_TYPE, content_type, ContentType::ReadsTable);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> EmbeddedFileBuilder<'a, 'b> {
    let start = _fbb.start_table();
    EmbeddedFileBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<EmbeddedFile<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for EmbeddedFile<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("EmbeddedFile");
      ds.field("offset", &self.offset());
      ds.field("length", &self.length());
      ds.field("format", &self.format());
      ds.field("content_type", &self.content_type());
      ds.finish()
  }
}
pub enum FooterOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct Footer<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Footer<'a> {
  type Inner = Footer<'a>;
  #[inline]
  unsafe fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table::new(buf, loc) }
  }
}

impl<'a> Footer<'a> {
  pub const VT_FILE_IDENTIFIER: flatbuffers::VOffsetT = 4;
  pub const VT_SOFTWARE: flatbuffers::VOffsetT = 6;
  pub const VT_POD5_VERSION: flatbuffers::VOffsetT = 8;
  pub const VT_CONTENTS: flatbuffers::VOffsetT = 10;

  #[inline]
  pub unsafe fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    Footer { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args FooterArgs<'args>
  ) -> flatbuffers::WIPOffset<Footer<'bldr>> {
    let mut builder = FooterBuilder::new(_fbb);
    if let Some(x) = args.contents { builder.add_contents(x); }
    if let Some(x) = args.pod5_version { builder.add_pod5_version(x); }
    if let Some(x) = args.software { builder.add_software(x); }
    if let Some(x) = args.file_identifier { builder.add_file_identifier(x); }
    builder.finish()
  }


  #[inline]
  pub fn file_identifier(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Footer::VT_FILE_IDENTIFIER, None)}
  }
  #[inline]
  pub fn software(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Footer::VT_SOFTWARE, None)}
  }
  #[inline]
  pub fn pod5_version(&self) -> Option<&'a str> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(Footer::VT_POD5_VERSION, None)}
  }
  #[inline]
  pub fn contents(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EmbeddedFile<'a>>>> {
    // Safety:
    // Created from valid Table for this object
    // which contains a valid value in this slot
    unsafe { self._tab.get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EmbeddedFile>>>>(Footer::VT_CONTENTS, None)}
  }
}

impl flatbuffers::Verifiable for Footer<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("file_identifier", Self::VT_FILE_IDENTIFIER, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("software", Self::VT_SOFTWARE, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("pod5_version", Self::VT_POD5_VERSION, false)?
     .visit_field::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<EmbeddedFile>>>>("contents", Self::VT_CONTENTS, false)?
     .finish();
    Ok(())
  }
}
#[derive(Default)]
pub struct FooterArgs<'a> {
    pub file_identifier: Option<flatbuffers::WIPOffset<&'a str>>,
    pub software: Option<flatbuffers::WIPOffset<&'a str>>,
    pub pod5_version: Option<flatbuffers::WIPOffset<&'a str>>,
    pub contents: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<EmbeddedFile<'a>>>>>,
}


pub struct FooterBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> FooterBuilder<'a, 'b> {
  #[inline]
  pub fn add_file_identifier(&mut self, file_identifier: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Footer::VT_FILE_IDENTIFIER, file_identifier);
  }
  #[inline]
  pub fn add_software(&mut self, software: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Footer::VT_SOFTWARE, software);
  }
  #[inline]
  pub fn add_pod5_version(&mut self, pod5_version: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Footer::VT_POD5_VERSION, pod5_version);
  }
  #[inline]
  pub fn add_contents(&mut self, contents: flatbuffers::WIPOffset<flatbuffers::Vector<'b , flatbuffers::ForwardsUOffset<EmbeddedFile<'b >>>>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(Footer::VT_CONTENTS, contents);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> FooterBuilder<'a, 'b> {
    let start = _fbb.start_table();
    FooterBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<Footer<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for Footer<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("Footer");
      ds.field("file_identifier", &self.file_identifier());
      ds.field("software", &self.software());
      ds.field("pod5_version", &self.pod5_version());
      ds.field("contents", &self.contents());
      ds.finish()
  }
}
#[inline]
/// Verifies that a buffer of bytes contains a `Footer`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_footer_unchecked`.
pub fn root_as_footer(buf: &[u8]) -> Result<Footer, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<Footer>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `Footer` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_footer_unchecked`.
pub fn size_prefixed_root_as_footer(buf: &[u8]) -> Result<Footer, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<Footer>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `Footer` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_footer_unchecked`.
pub fn root_as_footer_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<Footer<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `Footer` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_footer_unchecked`.
pub fn size_prefixed_root_as_footer_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<Footer<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<Footer<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a Footer and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `Footer`.
pub unsafe fn root_as_footer_unchecked(buf: &[u8]) -> Footer {
  flatbuffers::root_unchecked::<Footer>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed Footer and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `Footer`.
pub unsafe fn size_prefixed_root_as_footer_unchecked(buf: &[u8]) -> Footer {
  flatbuffers::size_prefixed_root_unchecked::<Footer>(buf)
}
#[inline]
pub fn finish_footer_buffer<'a>(
    fbb: &mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Footer<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_footer_buffer<'a>(fbb: &mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<Footer<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod ReadsFormat
}  // pub mod Minknow

