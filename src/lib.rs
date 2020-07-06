//! An error report handler for panics and the [`eyre`] crate for colorful, consistent, and well
//! formatted error reports for all kinds of errors.
//!
//! ## TLDR
//!
//! `color_eyre` helps you build error reports that look like this:
//!
//! <pre><font color="#06989A"><b>color-eyre</b></font> on <font color="#75507B"><b> hooked</b></font> <font color="#CC0000"><b>[$!] </b></font>is <font color="#FF8700"><b>📦 v0.5.0</b></font> via <font color="#CC0000"><b>🦀 v1.44.0</b></font>
//! <font color="#CC0000"><b>❯</b></font> cargo run --example custom_section
//! <font color="#4E9A06"><b>   Compiling</b></font> color-eyre v0.5.0 (/home/jlusby/git/yaahc/color-eyre)
//! <font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 2.58s
//! <font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/custom_section`
//! Error:
//!    0: <font color="#F15D22">Unable to read config</font>
//!    1: <font color="#F15D22">cmd exited with non-zero status code</font>
//!
//! Stderr:
//!    cat: fake_file: No such file or directory
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!
//!    0: <font color="#F15D22">custom_section::output2</font> with <font color="#34E2E2">self=&quot;cat&quot; &quot;fake_file&quot;</font>
//!       at examples/custom_section.rs:14
//!    1: <font color="#F15D22">custom_section::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
//!       at examples/custom_section.rs:58
//!    2: <font color="#F15D22">custom_section::read_config</font>
//!       at examples/custom_section.rs:63
//!
//! <font color="#34E2E2">Suggestion</font>: try using a file that exists next time</pre>
//!
//! ## Setup
//!
//! Add the following to your toml file:
//!
//! ```toml
//! [dependencies]
//! color-eyre = "0.5"
//! ```
//!
//! And install the panic and error report handlers:
//!
//! ```rust
//! use color_eyre::eyre::Result;
//!
//! fn main() -> Result<()> {
//!     color_eyre::install()?;
//!
//!     // ...
//!     # Ok(())
//! }
//! ```
//!
//! ### Disabling tracing support
//!
//! If you don't plan on using `tracing_error` and `SpanTrace` you can disable the
//! tracing integration to cut down on unused dependencies:
//!
//! ```toml
//! [dependencies]
//! color-eyre = { version = "0.5", default-features = false }
//! ```
//!
//! ### Disabling SpanTrace capture by default
//!
//! color-eyre defaults to capturing span traces. This is because `SpanTrace`
//! capture is significantly cheaper than `Backtrace` capture. However, like
//! backtraces, span traces are most useful for debugging applications, and it's
//! not uncommon to want to disable span trace capture by default to keep noise out
//! developer.
//!
//! To disable span trace capture you must explicitly set one of the env variables
//! that regulate `SpanTrace` capture to `"0"`:
//!
//! ```rust
//! if std::env::var("RUST_SPANTRACE").is_err() {
//!     std::env::set_var("RUST_SPANTRACE", "0");
//! }
//! ```
//!
//! ### Improving perf on debug builds
//!
//! In debug mode `color-eyre` behaves noticably worse than `eyre`. This is caused
//! by the fact that `eyre` uses `std::backtrace::Backtrace` instead of
//! `backtrace::Backtrace`. The std version of backtrace is precompiled with
//! optimizations, this means that whether or not you're in debug mode doesn't
//! matter much for how expensive backtrace capture is, it will always be in the
//! 10s of milliseconds to capture. A debug version of `backtrace::Backtrace`
//! however isn't so lucky, and can take an order of magnitude more time to capture
//! a backtrace compared to its std counterpart.
//!
//! Cargo [profile
//! overrides](https://doc.rust-lang.org/cargo/reference/profiles.html#overrides)
//! can be used to mitigate this problem. By configuring your project to always
//! build `backtrace` with optimizations you should get the same performance from
//! `color-eyre` that you're used to with `eyre`. To do so add the following to
//! your Cargo.toml:
//!
//! ```toml
//! [profile.dev.package.backtrace]
//! opt-level = 3
//! ```
//!
//! ## Features
//!
//! ### Multiple report format verbosity levels
//!
//! `color-eyre` provides 3 different report formats for how it formats the captured `SpanTrace`
//! and `Backtrace`, minimal, short, and full. Take the below snippets of the output produced by [`examples/usage.rs`]:
//!
//! ---
//!
//! Running `cargo run --example usage` without `RUST_LIB_BACKTRACE` set will produce a minimal
//! report like this:
//!
//! <pre><font color="#06989A"><b>color-eyre</b></font> on <font color="#75507B"><b> hooked</b></font> <font color="#CC0000"><b>[$!] </b></font>is <font color="#FF8700"><b>📦 v0.5.0</b></font> via <font color="#CC0000"><b>🦀 v1.44.0</b></font> took <font color="#C4A000"><b>2s</b></font>
//! <font color="#CC0000"><b>❯</b></font> cargo run --example usage
//! <font color="#4E9A06"><b>   Compiling</b></font> color-eyre v0.5.0 (/home/jlusby/git/yaahc/color-eyre)
//! <font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 2.23s
//! <font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/usage`
//! <font color="#A1A1A1">Jul 05 15:55:20.585 </font><font color="#4E9A06"> INFO</font> <b>read_config</b>:<b>read_file{</b>path=&quot;fake_file&quot;<b>}</b>: Reading file
//! Error:
//!    0: <font color="#F15D22">Unable to read config</font>
//!    1: <font color="#F15D22">No such file or directory (os error 2)</font>
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!
//!    0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
//!       at examples/usage.rs:32
//!    1: <font color="#F15D22">usage::read_config</font>
//!       at examples/usage.rs:38
//!
//! <font color="#34E2E2">Suggestion</font>: try using a file that exists next time</pre>
//!
//! <br>
//!
//! Running `RUST_LIB_BACKTRACE=1 cargo run --example usage` tells `color-eyre` to use the short
//! format, which additionally capture a [`backtrace::Backtrace`]:
//!
//! <pre><font color="#06989A"><b>color-eyre</b></font> on <font color="#75507B"><b> hooked</b></font> <font color="#CC0000"><b>[$!] </b></font>is <font color="#FF8700"><b>📦 v0.5.0</b></font> via <font color="#CC0000"><b>🦀 v1.44.0</b></font> took <font color="#C4A000"><b>2s</b></font>
//! <font color="#CC0000"><b>❯</b></font> RUST_LIB_BACKTRACE=1 cargo run --example usage
//! <font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.04s
//! <font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/usage`
//! <font color="#A1A1A1">Jul 05 15:55:25.187 </font><font color="#4E9A06"> INFO</font> <b>read_config</b>:<b>read_file{</b>path=&quot;fake_file&quot;<b>}</b>: Reading file
//! Error:
//!    0: <font color="#F15D22">Unable to read config</font>
//!    1: <font color="#F15D22">No such file or directory (os error 2)</font>
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!
//!    0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
//!       at examples/usage.rs:32
//!    1: <font color="#F15D22">usage::read_config</font>
//!       at examples/usage.rs:38
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!   <font color="#34E2E2">                              ⋮ 5 frames hidden ⋮                               </font>
//!    6: <font color="#F15D22">usage::read_file</font><font color="#88807C">::h1a5a2805bc32aea2</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">35</font>
//!    7: <font color="#F15D22">usage::read_config</font><font color="#88807C">::h2bd40d6b0bb869a5</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">40</font>
//!    8: <font color="#F15D22">usage::main</font><font color="#88807C">::h899ba01ae5d58327</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">11</font>
//!   <font color="#34E2E2">                              ⋮ 10 frames hidden ⋮                              </font>
//!
//! <font color="#34E2E2">Suggestion</font>: try using a file that exists next time</pre>
//!
//! <br>
//!
//! Finally, running `RUST_LIB_BACKTRACE=full cargo run --example usage` tells `color-eyre` to use
//! the full format, which in addition to the above will attempt to include source lines where the
//! error originated from, assuming it can find them on the disk.
//!
//! <pre><font color="#06989A"><b>color-eyre</b></font> on <font color="#75507B"><b> hooked</b></font> <font color="#CC0000"><b>[$!] </b></font>is <font color="#FF8700"><b>📦 v0.5.0</b></font> via <font color="#CC0000"><b>🦀 v1.44.0</b></font> took <font color="#C4A000"><b>6s</b></font>
//! <font color="#CC0000"><b>❯</b></font> RUST_LIB_BACKTRACE=full cargo run --example usage
//! <font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 0.08s
//! <font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/usage`
//! <font color="#A1A1A1">Jul 05 16:02:53.110 </font><font color="#4E9A06"> INFO</font> <b>read_config</b>:<b>read_file{</b>path=&quot;fake_file&quot;<b>}</b>: Reading file
//! Error:
//!    0: <font color="#F15D22">Unable to read config</font>
//!    1: <font color="#F15D22">No such file or directory (os error 2)</font>
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!
//!    0: <font color="#F15D22">usage::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
//!       at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">32</font>
//!         30 │ }
//!         31 │
//!   <b>      32 &gt; #[instrument]</b>
//!         33 │ fn read_file(path: &amp;str) -&gt; Result&lt;(), Report&gt; {
//!         34 │     info!(&quot;Reading file&quot;);
//!    1: <font color="#F15D22">usage::read_config</font>
//!       at <font color="#75507B">examples/usage.rs</font>:<font color="#75507B">38</font>
//!         36 │ }
//!         37 │
//!   <b>      38 &gt; #[instrument]</b>
//!         39 │ fn read_config() -&gt; Result&lt;(), Report&gt; {
//!         40 │     read_file(&quot;fake_file&quot;)
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ BACKTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!   <font color="#34E2E2">                              ⋮ 5 frames hidden ⋮                               </font>
//!    6: <font color="#F15D22">usage::read_file</font><font color="#88807C">::hf2b3585728b26de3</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">35</font>
//!         33 │ fn read_file(path: &amp;str) -&gt; Result&lt;(), Report&gt; {
//!         34 │     info!(&quot;Reading file&quot;);
//!   <font color="#D3D7CF"><b>      35 &gt;     Ok(std::fs::read_to_string(path).map(drop)?)</b></font>
//!         36 │ }
//!         37 │
//!    7: <font color="#F15D22">usage::read_config</font><font color="#88807C">::h399e4483ed46d301</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">40</font>
//!         38 │ #[instrument]
//!         39 │ fn read_config() -&gt; Result&lt;(), Report&gt; {
//!   <font color="#D3D7CF"><b>      40 &gt;     read_file(&quot;fake_file&quot;)</b></font>
//!         41 │         .wrap_err(&quot;Unable to read config&quot;)
//!         42 │         .suggestion(&quot;try using a file that exists next time&quot;)
//!    8: <font color="#F15D22">usage::main</font><font color="#88807C">::h037e068f8ce9bf49</font>
//!       at <font color="#75507B">/home/jlusby/git/yaahc/color-eyre/examples/usage.rs</font>:<font color="#75507B">11</font>
//!          9 │     color_eyre::install()?;
//!         10 │
//!   <font color="#D3D7CF"><b>      11 &gt;     Ok(read_config()?)</b></font>
//!         12 │ }
//!         13 │
//!   <font color="#34E2E2">                              ⋮ 10 frames hidden ⋮                              </font>
//!
//! <font color="#34E2E2">Suggestion</font>: try using a file that exists next time</pre>
//!
//! ### Custom `Section`s for error reports via [`Help`] trait
//!
//! The `section` module provides helpers for adding extra sections to error
//! reports. Sections are disinct from error messages and are displayed
//! independently from the chain of errors. Take this example of adding sections
//! to contain `stderr` and `stdout` from a failed command, taken from
//! [`examples/custom_section.rs`]:
//!
//! ```rust
//! use color_eyre::{eyre::eyre, SectionExt, Help, eyre::Report};
//! use std::process::Command;
//! use tracing::instrument;
//!
//! trait Output {
//!     fn output2(&mut self) -> Result<String, Report>;
//! }
//!
//! impl Output for Command {
//!     #[instrument]
//!     fn output2(&mut self) -> Result<String, Report> {
//!         let output = self.output()?;
//!
//!         let stdout = String::from_utf8_lossy(&output.stdout);
//!
//!         if !output.status.success() {
//!             let stderr = String::from_utf8_lossy(&output.stderr);
//!             Err(eyre!("cmd exited with non-zero status code"))
//!                 .with_section(move || stdout.trim().to_string().header("Stdout:"))
//!                 .with_section(move || stderr.trim().to_string().header("Stderr:"))
//!         } else {
//!             Ok(stdout.into())
//!         }
//!     }
//! }
//! ```
//!
//! ---
//!
//! Here we have an function that, if the command exits unsuccessfully, creates a
//! report indicating the failure and attaches two sections, one for `stdout` and
//! one for `stderr`.
//!
//! Running `cargo run --example custom_section` shows us how these sections are
//! included in the output:
//!
//! <pre><font color="#06989A"><b>color-eyre</b></font> on <font color="#75507B"><b> hooked</b></font> <font color="#CC0000"><b>[$!] </b></font>is <font color="#FF8700"><b>📦 v0.5.0</b></font> via <font color="#CC0000"><b>🦀 v1.44.0</b></font>
//! <font color="#CC0000"><b>❯</b></font> cargo run --example custom_section
//! <font color="#4E9A06"><b>   Compiling</b></font> color-eyre v0.5.0 (/home/jlusby/git/yaahc/color-eyre)
//! <font color="#4E9A06"><b>    Finished</b></font> dev [unoptimized + debuginfo] target(s) in 2.58s
//! <font color="#4E9A06"><b>     Running</b></font> `target/debug/examples/custom_section`
//! Error:
//!    0: <font color="#F15D22">Unable to read config</font>
//!    1: <font color="#F15D22">cmd exited with non-zero status code</font>
//!
//! Stderr:
//!    cat: fake_file: No such file or directory
//!
//!   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ SPANTRACE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//!
//!    0: <font color="#F15D22">custom_section::output2</font> with <font color="#34E2E2">self=&quot;cat&quot; &quot;fake_file&quot;</font>
//!       at examples/custom_section.rs:14
//!    1: <font color="#F15D22">custom_section::read_file</font> with <font color="#34E2E2">path=&quot;fake_file&quot;</font>
//!       at examples/custom_section.rs:58
//!    2: <font color="#F15D22">custom_section::read_config</font>
//!       at examples/custom_section.rs:63
//!
//! <font color="#34E2E2">Suggestion</font>: try using a file that exists next time</pre>
//!
//! Only the `Stderr:` section actually gets included. The `cat` command fails,
//! so stdout ends up being empty and is skipped in the final report. This gives
//! us a short and concise error report indicating exactly what was attempted and
//! how it failed.
//!
//! ### Aggregating multiple errors into one report
//!
//! It's not uncommon for programs like batched task runners or parsers to want
//! to return an error with multiple sources. The current version of the error
//! trait does not support this use case very well, though there is [work being
//! done](https://github.com/rust-lang/rfcs/pull/2895) to improve this.
//!
//! For now however one way to work around this is to compose errors outside the
//! error trait. `color-eyre` supports such composition in its error reports via
//! the `Help` trait.
//!
//! For an example of how to aggregate errors check out [`examples/multiple_errors.rs`].
//!
//! ### Custom configuration for `color-backtrace` for setting custom filters and more
//!
//! The pretty printing for backtraces and span traces isn't actually provided by
//! `color-eyre`, but instead comes from its dependencies [`color-backtrace`] and
//! [`color-spantrace`]. `color-backtrace` in particular has many more features
//! than are exported by `color-eyre`, such as customized color schemes, panic
//! hooks, and custom frame filters. The custom frame filters are particularly
//! useful when combined with `color-eyre`, so to enable their usage we provide
//! the `install` fn for setting up a custom `BacktracePrinter` with custom
//! filters installed.
//!
//! For an example of how to setup custom filters, check out [`examples/custom_filter.rs`].
//!
//! [`eyre`]: https://docs.rs/eyre
//! [`tracing-error`]: https://docs.rs/tracing-error
//! [`color-backtrace`]: https://docs.rs/color-backtrace
//! [`eyre::EyreHandler`]: https://docs.rs/eyre/*/eyre/trait.EyreHandler.html
//! [`backtrace::Backtrace`]: https://docs.rs/backtrace/*/backtrace/struct.Backtrace.html
//! [`tracing_error::SpanTrace`]: https://docs.rs/tracing-error/*/tracing_error/struct.SpanTrace.html
//! [`color-spantrace`]: https://github.com/yaahc/color-spantrace
//! [`Help`]: https://docs.rs/color-eyre/*/color_eyre/trait.Help.html
//! [`eyre::Report`]: https://docs.rs/eyre/*/eyre/struct.Report.html
//! [`eyre::Result`]: https://docs.rs/eyre/*/eyre/type.Result.html
//! [`Handler`]: https://docs.rs/color-eyre/*/color_eyre/struct.Handler.html
//! [`examples/usage.rs`]: https://github.com/yaahc/color-eyre/blob/master/examples/usage.rs
//! [`examples/custom_filter.rs`]: https://github.com/yaahc/color-eyre/blob/master/examples/custom_filter.rs
//! [`examples/custom_section.rs`]: https://github.com/yaahc/color-eyre/blob/master/examples/custom_section.rs
//! [`examples/multiple_errors.rs`]: https://github.com/yaahc/color-eyre/blob/master/examples/multiple_errors.rs
#![doc(html_root_url = "https://docs.rs/color-eyre/0.5.0")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(
    missing_docs,
    missing_doc_code_examples,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![allow(clippy::try_err)]
use backtrace::Backtrace;
pub use eyre;
#[doc(hidden)]
pub use eyre::Report;
#[doc(hidden)]
pub use eyre::Result;
use once_cell::sync::OnceCell;
use section::help::HelpInfo;
#[doc(hidden)]
pub use section::Section as Help;
pub use section::{IndentedSection, Section, SectionExt};
#[cfg(feature = "capture-spantrace")]
use tracing_error::SpanTrace;
#[doc(hidden)]
pub use Handler as Context;

pub mod config;
mod handler;
pub(crate) mod private;
pub mod section;
mod writers;

/// A custom handler type for [`eyre::Report`] which provides colorful error
/// reports and [`tracing-error`] support.
///
/// # Details
///
/// This type is not intended to be used directly, prefer using it via the
/// [`color_eyre::Report`] and [`color_eyre::Result`] type aliases.
///
/// [`eyre::Report`]: https://docs.rs/eyre/*/eyre/struct.Report.html
/// [`tracing-error`]: https://docs.rs/tracing-error
/// [`color_eyre::Report`]: type.Report.html
/// [`color_eyre::Result`]: type.Result.html
#[derive(Debug)]
pub struct Handler {
    backtrace: Option<Backtrace>,
    #[cfg(feature = "capture-spantrace")]
    span_trace: Option<SpanTrace>,
    sections: Vec<HelpInfo>,
}

static CONFIG: OnceCell<config::PanicHook> = OnceCell::new();

// TODO: remove when / if ansi_term merges these changes upstream
trait ColorExt {
    fn make_intense(self) -> Self;
}

/// Install the default panic and error report hooks
///
/// # Details
///
/// This function must be called to enable the customization of `eyre::Report`
/// provided by `color-eyre`. This function should be called early, ideally
/// before any errors could be encountered.
///
/// Only the first install will succeed. Calling this function after another
/// report handler has been installed will cause an error. **Note**: This
/// function _must_ be called before any `eyre::Report`s are constructed to
/// prevent the default handler from being installed.
///
/// # Examples
///
/// ```rust
/// use color_eyre::eyre::Result;
///
/// fn main() -> Result<()> {
///     color_eyre::install()?;
///
///     // ...
///     # Ok(())
/// }
/// ```
pub fn install() -> Result<(), crate::eyre::Report> {
    config::HookBuilder::default().install()
}
