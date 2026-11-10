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
//! the totals of the whole record, under GJ-11. The juror
//! ledger reading reads the whole record respondent by
//! respondent and states the counts of each respondent, the
//! movement of its own answers, and how often its answer
//! stood with the plurality of its jury, under GJ-13. The
//! dissent reading reads the whole record asking by
//! asking and states the plurality of each jury, the
//! margin it stood by, and the respondents who stood
//! against it, under GJ-15. The bench reading reads the
//! whole record model by model and states the plurality
//! of each model at each asking pooled over the juror
//! names it answered under, the count of its valid
//! answers, and how many of them stood with the pooled
//! plurality of the asking, under GJ-23. The swing reading reads the
//! whole record question by question and states the
//! plurality of each asking pooled over the valid
//! responses of the whole poll, the margin it stood by,
//! and the course of each question across its askings,
//! under GJ-17. The minority reading reads the whole
//! record question by question and states the plurality
//! of each asking pooled over the whole poll, the margin
//! it stood by, and the respondents whose valid answer
//! was not the pooled plurality answer, under GJ-19. The
//! pool reading reads the whole record question by
//! question and states the pooled counts of each asking,
//! the count and the share of every offered answer among
//! its pooled valid responses, and its pooled plurality,
//! under GJ-25. The aggregate reading reads the whole
//! record question by question and states for each
//! question the counts of all its askings summed as one
//! pool, the count and the share of every offered answer
//! of the question among its pooled valid responses, and
//! the pooled plurality of the question, under GJ-27. The
//! canvass reading reads the whole record question by
//! question and states for each question, per jury, the
//! counts of the jury summed over the askings of the
//! question, the count and the share of every offered
//! answer of the question among the summed valid responses
//! of the jury, and the plurality of the jury at the
//! question, under GJ-29. The roll reading reads the
//! whole record jury by jury and states for each jury
//! the block of the jury at every question it stands at,
//! the counts of the jury summed over the askings of the
//! question, the count and the share of every offered
//! answer of the question among the summed valid responses
//! of the jury, and the plurality of the jury at the
//! question, with the standing of the jury over the
//! record closing its section, under GJ-31. The
//! muster reading reads the whole record question by
//! question and states for each question, under each
//! offered answer of the question, the row of that
//! answer in the block of every jury of the question,
//! the count of the valid responses of the question
//! that chose the answer, the count of the juries whose
//! summed count of the answer stands at one or more,
//! and the count of the juries whose block plurality
//! the answer is, under GJ-33. The margin reading
//! reads the whole record question by question and
//! states for each question the margin by which the
//! pooled plurality of the question stands and the
//! margin by which the summed plurality of each jury
//! at the question stands, under GJ-35. The
//! forms the readings across the recorded sets share live
//! in record. The command line surface lives in cli, the
//! ported maintainer makers live in makers, and the
//! codepoint scan lives in scan.

pub mod aggregate;
pub mod agreement;
pub mod bench;
pub mod canvass;
pub mod census;
pub mod cli;
pub mod compare;
pub mod dissent;
pub mod json;
pub mod ledger;
pub mod makers;
pub mod margin;
pub mod minority;
pub mod muster;
pub mod polls;
pub mod pool;
pub mod pydecimal;
pub mod pynum;
pub mod pytext;
pub mod pyunicode;
pub mod record;
pub mod roll;
pub mod scan;
pub mod seats;
pub mod swing;
