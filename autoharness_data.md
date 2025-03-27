| Functions with Automatic Harnesses | Count |
| ---------------------------------- | ----- |
| Safe Functions                     | 5384  |
| Safe Abstractions                  | 876   |
| Unsafe Functions                   | 598   |
| Total                              | 6858  |

| Reason function was skipped                      | # of functions skipped for this reason |
| ------------------------------------------------ | -------------------------------------- |
| Missing Arbitrary implementation for argument(s) | 17912                                  |
| Generic Function                                 | 13652                                  |
| Kani implementation                              | 1972                                   |
| The function does not have a body                | 102                                    |
| Total                                            | 33638                                  |

| Unsupported Type Category                                                                                                        | # of occurences |
| -------------------------------------------------------------------------------------------------------------------------------- | --------------- |
| &                                                                                                                                | 11131           |
| core_arch::x86                                                                                                                   | 8214            |
| &mut                                                                                                                             | 4100            |
| num::wrapping                                                                                                                    | 744             |
| num::saturating                                                                                                                  | 684             |
| *const                                                                                                                           | 599             |
| core_simd::vector                                                                                                                | 404             |
| *mut                                                                                                                             | 367             |
| sync::atomic                                                                                                                     | 189             |
| other                                                                                                                            | 78              |
| core_arch::simd                                                                                                                  | 53              |
| read::value                                                                                                                      | 47              |
| net::ip_addr                                                                                                                     | 44              |
| core::io                                                                                                                         | 32              |
| constants                                                                                                                        | 26              |
| bridge::client                                                                                                                   | 22              |
| process                                                                                                                          | 21              |
| string                                                                                                                           | 21              |
| range                                                                                                                            | 19              |
| common                                                                                                                           | 19              |
| os::fd::owned                                                                                                                    | 19              |
| num::niche_types                                                                                                                 | 18              |
| sys::fs::unix                                                                                                                    | 17              |
| ffi::c_str                                                                                                                       | 16              |
| ascii::ascii_char                                                                                                                | 16              |
| math::libm::support::big                                                                                                         | 16              |
| int::big                                                                                                                         | 15              |
| time                                                                                                                             | 15              |
| path                                                                                                                             | 15              |
| sys::pal::unix::fd                                                                                                               | 14              |
| char                                                                                                                             | 14              |
| rustc_std_workspace_core::arch::x86_64                                                                                           | 14              |
| fmt                                                                                                                              | 13              |
| core::fmt                                                                                                                        | 12              |
| ffi::os_str                                                                                                                      | 12              |
| sys::pal::unix::process::process_common                                                                                          | 12              |
| pe                                                                                                                               | 12              |
| collections::hash::map                                                                                                           | 11              |
| vec                                                                                                                              | 11              |
| fs                                                                                                                               | 11              |
| vector                                                                                                                           | 10              |
| sys::net::connection::socket                                                                                                     | 10              |
| str::iter                                                                                                                        | 10              |
| cmp                                                                                                                              | 10              |
| read::pe::resource                                                                                                               | 10              |
| map                                                                                                                              | 9               |
| thread                                                                                                                           | 9               |
| borrow                                                                                                                           | 9               |
| str                                                                                                                              | 9               |
| ops::range                                                                                                                       | 8               |
| sync::mpmc::select                                                                                                               | 8               |
| mem::transmutability                                                                                                             | 8               |
| sys::net::connection::socket::unix                                                                                               | 8               |
| rustc_std_workspace_core::option                                                                                                 | 8               |
| sys_common::wtf8                                                                                                                 | 8               |
| alloc_crate::vec                                                                                                                 | 8               |
| task::wake                                                                                                                       | 8               |
| sys::pal::unix::pipe                                                                                                             | 8               |
| control::tag                                                                                                                     | 8               |
| libc                                                                                                                             | 8               |
| control::bitmask                                                                                                                 | 8               |
| io                                                                                                                               | 7               |
| read                                                                                                                             | 7               |
| net::tcp                                                                                                                         | 7               |
| escape                                                                                                                           | 7               |
| intrinsics::mir                                                                                                                  | 6               |
| sys::os_str::bytes                                                                                                               | 6               |
| alloc_crate::string                                                                                                              | 6               |
| sys::pal::unix::process::process_inner                                                                                           | 6               |
| cell                                                                                                                             | 6               |
| core::ops                                                                                                                        | 6               |
| io::pipe                                                                                                                         | 6               |
| read::cfi                                                                                                                        | 6               |
| control::group::generic                                                                                                          | 6               |
| diagnostic                                                                                                                       | 5               |
| arch::all::packedpair                                                                                                            | 5               |
| sys::pal::unix::time                                                                                                             | 5               |
| arch::all::twoway                                                                                                                | 5               |
| io::error                                                                                                                        | 5               |
| io::stdio                                                                                                                        | 5               |
| fmt::rt                                                                                                                          | 5               |
| raw                                                                                                                              | 5               |
| net                                                                                                                              | 5               |
| bridge::api_tags                                                                                                                 | 5               |
| boxed                                                                                                                            | 5               |
| collections::hash::set                                                                                                           | 4               |
| math::libm::support::env                                                                                                         | 4               |
| backtrace_rs::print                                                                                                              | 4               |
| option::Option<fmt                                                                                                               | 4               |
| rustc_std_workspace_core::option::Option<memmem::searcher                                                                        | 4               |
| core::result::Result<&core::net::SocketAddr, io::error                                                                           | 4               |
| net::socket_addr                                                                                                                 | 4               |
| core::option::Option<backtrace_rs::symbolize                                                                                     | 3               |
| unwind                                                                                                                           | 3               |
| sys::pal::unix::kernel_copy                                                                                                      | 3               |
| os::unix::net::ancillary                                                                                                         | 3               |
| collections::linked_list                                                                                                         | 3               |
| bridge                                                                                                                           | 3               |
| os::linux::process                                                                                                               | 3               |
| std::option::Option<bridge::client                                                                                               | 3               |
| str::pattern                                                                                                                     | 3               |
| alloc_crate::boxed::Box<dyn core::any::Any + core::marker                                                                        | 3               |
| bridge::buffer                                                                                                                   | 3               |
| core::option::Option<backtrace_rs::types                                                                                         | 3               |
| net::udp                                                                                                                         | 3               |
| inflate::core                                                                                                                    | 3               |
| read::abbrev                                                                                                                     | 3               |
| detect::cache                                                                                                                    | 3               |
| marker::variance                                                                                                                 | 3               |
| bstr                                                                                                                             | 3               |
| sys::stdio::unix                                                                                                                 | 3               |
| core::pin::Pin<&sys::sync::thread_parking::futex                                                                                 | 3               |
| std::string                                                                                                                      | 3               |
| endian                                                                                                                           | 3               |
| endianity                                                                                                                        | 3               |
| sys::io::io_slice::iovec                                                                                                         | 3               |
| memmem                                                                                                                           | 2               |
| alloc_crate::boxed::Box<dyn for<'a, 'b> core::ops::Fn(&'a panic::PanicHookInfo<'b>) + core::marker::Send + core::marker          | 2               |
| boxed::Box<core::ffi                                                                                                             | 2               |
| os::unix::net::listener                                                                                                          | 2               |
| core::option                                                                                                                     | 2               |
| core::option::Option<&path                                                                                                       | 2               |
| io::borrowed_buf                                                                                                                 | 2               |
| sys::pal::unix::linux::pidfd                                                                                                     | 2               |
| core::option::Option<alloc_crate::sync::Arc<sync::poison::mutex::Mutex<alloc_crate::vec                                          | 2               |
| slice::sort::stable::drift                                                                                                       | 2               |
| alloc_crate::ffi                                                                                                                 | 2               |
| borrow::Cow<'_, core::ffi                                                                                                        | 2               |
| collections                                                                                                                      | 2               |
| panic                                                                                                                            | 2               |
| alloc_crate::borrow::Cow<'_, ffi::os_str                                                                                         | 2               |
| std::vec                                                                                                                         | 2               |
| read::index                                                                                                                      | 2               |
| std::option                                                                                                                      | 2               |
| core::option::Option<&sys::pal::unix::process::process_common                                                                    | 2               |
| rc                                                                                                                               | 2               |
| float::cmp                                                                                                                       | 2               |
| sync::mpsc                                                                                                                       | 2               |
| set                                                                                                                              | 2               |
| detect::arch::x86                                                                                                                | 2               |
| sys::pal::unix::thread                                                                                                           | 2               |
| panicking                                                                                                                        | 2               |
| os::unix::net::datagram                                                                                                          | 2               |
| read::util                                                                                                                       | 2               |
| sync::poison::once                                                                                                               | 2               |
| alloc_crate::boxed::Box<ffi::os_str                                                                                              | 2               |
| convert                                                                                                                          | 2               |
| alloc_crate::borrow::Cow<'_, path                                                                                                | 2               |
| os::unix::net::stream                                                                                                            | 2               |
| sync                                                                                                                             | 2               |
| arch::all::rabinkarp                                                                                                             | 2               |
| slice::ascii                                                                                                                     | 2               |
| alloc::vec::Vec<read::abbrev                                                                                                     | 2               |
| ascii                                                                                                                            | 2               |
| alloc_crate::boxed::Box<path                                                                                                     | 2               |
| backtrace_rs::types                                                                                                              | 2               |
| core::option::Option<backtrace_rs::symbolize::gimli::elf                                                                         | 2               |
| vec::Vec<core::num                                                                                                               | 1               |
| core::sync::atomic                                                                                                               | 1               |
| alloc::borrow                                                                                                                    | 1               |
| bridge::Group<bridge::client::TokenStream, bridge::client                                                                        | 1               |
| read::lists                                                                                                                      | 1               |
| future                                                                                                                           | 1               |
| core::option::Option<time                                                                                                        | 1               |
| backtrace_rs::symbolize::gimli::elf                                                                                              | 1               |
| read::reader                                                                                                                     | 1               |
| gimli                                                                                                                            | 1               |
| backtrace_rs::symbolize::gimli::mmap                                                                                             | 1               |
| bridge::Punct<bridge::client                                                                                                     | 1               |
| hashbrown                                                                                                                        | 1               |
| arch::all::memchr                                                                                                                | 1               |
| bridge::symbol                                                                                                                   | 1               |
| net::parser                                                                                                                      | 1               |
| arch::generic::memchr::Three<rustc_std_workspace_core::arch::x86_64                                                              | 1               |
| core::hash                                                                                                                       | 1               |
| v0                                                                                                                               | 1               |
| backtrace_rs::backtrace                                                                                                          | 1               |
| option                                                                                                                           | 1               |
| bridge::Diagnostic<bridge::client                                                                                                | 1               |
| thread::spawnhook                                                                                                                | 1               |
| algo                                                                                                                             | 1               |
| legacy                                                                                                                           | 1               |
| core::option::Option<thread                                                                                                      | 1               |
| std::vec::Vec<bridge::client                                                                                                     | 1               |
| sync::lazy_lock::LazyLock<backtrace                                                                                              | 1               |
| num::flt2dec::decoder                                                                                                            | 1               |
| alloc_crate::boxed::Box<io::error                                                                                                | 1               |
| endian::U64Bytes<endian                                                                                                          | 1               |
| any                                                                                                                              | 1               |
| alloc_crate::boxed::Box<sys_common::wtf8                                                                                         | 1               |
| unsafe fn(*const ()) -> task::wake                                                                                               | 1               |
| read::archive                                                                                                                    | 1               |
| char::methods                                                                                                                    | 1               |
| arch::x86_64::sse2::memchr                                                                                                       | 1               |
| core::convert                                                                                                                    | 1               |
| alloc::collections::BTreeMap<u64, read                                                                                           | 1               |
| for<'a, 'b, 'c> fn(&'a str, &'b mut core::fmt::Formatter<'c>) -> core::result::Result<(), core::fmt                              | 1               |
| memchr                                                                                                                           | 1               |
| boxed::Box<core::bstr                                                                                                            | 1               |
| core::ascii                                                                                                                      | 1               |
| token_stream                                                                                                                     | 1               |
| thread::local                                                                                                                    | 1               |
| rustc_std_workspace_core::option::Option<gimli                                                                                   | 1               |
| std::slice::Iter<'_, diagnostic                                                                                                  | 1               |
| os::unix::net::ancillary::AncillaryDataIter<'_, libc                                                                             | 1               |
| bridge::rpc                                                                                                                      | 1               |
| sync::poison::mutex                                                                                                              | 1               |
| arch::generic::memchr::Two<rustc_std_workspace_core::arch::x86_64                                                                | 1               |
| alloc_crate::boxed::Box<sys::os_str::bytes                                                                                       | 1               |
| alloc_crate::collections::BTreeMap<ffi::os_str::OsString, ffi::os_str                                                            | 1               |
| cow                                                                                                                              | 1               |
| error::Tagged<dyn error                                                                                                          | 1               |
| core::alloc                                                                                                                      | 1               |
| core::option::Option<path                                                                                                        | 1               |
| arch::generic::packedpair::Finder<rustc_std_workspace_core::arch::x86_64                                                         | 1               |
| array::iter                                                                                                                      | 1               |
| rustc_std_workspace_core::sync::atomic                                                                                           | 1               |
| num::flt2dec                                                                                                                     | 1               |
| arch::generic::memchr::One<rustc_std_workspace_core::arch::x86_64                                                                | 1               |
| iter::adapters::map::Map<str::iter::SplitInclusive<'_, char>, str                                                                | 1               |
| inflate                                                                                                                          | 1               |
| io::buffered::bufwriter                                                                                                          | 1               |
| io::error::repr_bitpacked                                                                                                        | 1               |
| alloc_crate::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker                                               | 1               |
| bridge::Ident<bridge::client::Span, bridge::symbol                                                                               | 1               |
| core::option::Option<alloc_crate::string                                                                                         | 1               |
| alloc_crate::collections                                                                                                         | 1               |
| sync::Arc<core::bstr                                                                                                             | 1               |
| backtrace                                                                                                                        | 1               |
| lazy::LazyCell<rustc_std_workspace_core::result::Result<line::Lines, gimli                                                       | 1               |
| arch::x86_64::avx2::memchr                                                                                                       | 1               |
| memmem::searcher                                                                                                                 | 1               |
| std::vec::Vec<bridge::TokenTree<bridge::client::TokenStream, bridge::client::Span, bridge::symbol                                | 1               |
| collections::hash::set::Intersection<'_, &str, hash::random                                                                      | 1               |
| std::boxed::Box<dyn std::any::Any + std::marker                                                                                  | 1               |
| bridge::Literal<bridge::client::Span, bridge::symbol                                                                             | 1               |
| io::error::ErrorData<alloc_crate::boxed::Box<io::error                                                                           | 1               |
| bridge::TokenTree<bridge::client::TokenStream, bridge::client::Span, bridge::symbol                                              | 1               |
| hash::sip                                                                                                                        | 1               |
| collections::hash::set::Union<'_, &str, hash::random                                                                             | 1               |
| endian::U32Bytes<endian                                                                                                          | 1               |
| iter::adapters::copied::Copied<slice::iter                                                                                       | 1               |
| addr2line::SplitDwarfLoad<addr2line::gimli::EndianSlice<'_, addr2line::gimli                                                     | 1               |
| (sys::pal::unix::process::process_inner::Process, sys::pal::unix::process::process_common                                        | 1               |
| collections::hash::set::Difference<'_, &str, hash::random                                                                        | 1               |
| std::vec::IntoIter<bridge::TokenTree<bridge::client::TokenStream, bridge::client::Span, bridge::symbol                           | 1               |
| fn(core::alloc                                                                                                                   | 1               |
| pin::Pin<&mut future::async_drop                                                                                                 | 1               |
| alloc::boxed::Box<dyn core::any::Any + core::marker                                                                              | 1               |
| collections::hash::set::SymmetricDifference<'_, &str, hash::random                                                               | 1               |
| alloc_crate::boxed::Box<dyn core::ops::FnMut() -> core::result::Result<(), io::error::Error> + core::marker::Send + core::marker | 1               |
| core::option::Option<&addr2line::gimli::DwarfPackage<addr2line::gimli::EndianSlice<'_, addr2line::gimli                          | 1               |
| core::pin::Pin<&thread                                                                                                           | 1               |
| alloc_crate::boxed::Box<dyn core::ops                                                                                            | 1               |
| backtrace_rs::symbolize                                                                                                          | 1               |
| rc::Rc<core::bstr                                                                                                                | 1               |
| Total                                                                                                                            | 2610            |

