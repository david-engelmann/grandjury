//! grandjury, poll tiny models and render the stadium vote.
//!
//! The library carries the whole surface. The poll loader
//! reads a recorded poll under the defect rules of GJ-1 and
//! GJ-2 and never raises. The tally and the stadium view
//! render a loaded poll under GJ-3, the tally holding exact
//! fractions of the valid responses. The cost report renders
//! a poll priced by a table of decimal strings under GJ-4, at
//! totals of six decimals with halves rounded up. The seats
//! module reads one recorded answer envelope of each route as
//! a poll response under GJ-5. The agreement reading compares
//! the juries of several polls by the model that produced each
//! response, under GJ-8. The wave comparison reading pairs two
//! recorded waves and states the movement of each jury
//! plurality between them, under GJ-10. The census reading
//! reads across the recorded poll sets and states the
//! plurality of each jury at each asking of each question and
//! the totals of the whole record, under GJ-11. The command
//! line surface lives in cli, the ported maintainer makers
//! live in makers, and the codepoint scan lives in scan.

pub mod agreement;
pub mod census;
pub mod cli;
pub mod compare;
pub mod json;
pub mod makers;
pub mod polls;
pub mod pydecimal;
pub mod pynum;
pub mod pytext;
pub mod pyunicode;
pub mod scan;
pub mod seats;
