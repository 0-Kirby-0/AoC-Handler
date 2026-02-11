#![allow(unused)]
#[allow(clippy::wildcard_imports)]
use super::*;
use crate::{
    SolutionPart,
    execution::{
        result::{
            AcquisitionError, CheckReturn, CheckedRunReturn, PartOutput, RunReturn, Unchecked,
        },
        solution_part::CheckError,
    },
};

#[test]
fn print_year() {
    println!(
        "[1m[4mRunning[22m[24m {}",
        render_year_returns(
            TimeKey::<TimeDetailNone>::new(2025).unwrap(),
            generate_day_return_variants()
        )
    );
}
#[test]
fn print_days() {
    for (i, dr) in generate_day_return_variants().enumerate() {
        println!(
            "[1m[4mRunning[22m[24m {}\n",
            render_day_return(
                TimeKey::<TimeDetailDay>::new(2024, (i + 1).min(25).try_into().unwrap()).unwrap(),
                &dr
            )
        );
    }
}

#[test]
fn print_parts() {
    for (i, part_return) in generate_part_return_variants().enumerate() {
        println!(
            "[1m[4mRunning[22m[24m {}\n",
            render_part_return(
                TimeKey::<TimeDetailDayAndPart>::new(2024, (i + 1).min(25).try_into().unwrap(), 2)
                    .unwrap(),
                &part_return
            )
        );
    }
}

fn generate_day_return_variants() -> impl Iterator<Item = DayReturn> {
    itertools::Itertools::tuples(generate_part_return_variants()).map(|tup: (_, _)| tup.into())
}

fn generate_part_return_variants() -> impl Iterator<Item = PartReturn> {
    let mandatory_solution_parts = [
        SolutionPart::from(rust_decimal::Decimal::MAX),
        SolutionPart::from(true),
        SolutionPart::from("consectetur"),
    ];

    let solution_parts = mandatory_solution_parts
        .into_iter()
        .chain(std::iter::from_fn(|| Some(SolutionPart::from(1234))));

    let check_errors = [
        CheckError::WrongFormat(0.into(), "quid novi".into()),
        CheckError::Incorrect(0.into(), 1.into(), Some(std::cmp::Ordering::Less)),
        CheckError::Incorrect(1.into(), 0.into(), Some(std::cmp::Ordering::Greater)),
        CheckError::Incorrect("lorem".into(), "ipsum".into(), None),
    ];

    let check_returns = [
        CheckReturn::Unchecked(Unchecked::Elided),
        CheckReturn::Unchecked(Unchecked::ElideMismatch("tempor incididunt".into())),
        CheckReturn::Unchecked(Unchecked::MissingInput),
        CheckReturn::Unchecked(Unchecked::MissingOuput("dolor sit amet".into())),
        CheckReturn::Passed,
        CheckReturn::Passed,
        CheckReturn::Passed,
        CheckReturn::Passed,
        CheckReturn::Passed,
        CheckReturn::Passed,
        CheckReturn::Passed,
    ]
    .into_iter()
    .chain(check_errors.into_iter().map(CheckReturn::Failed));

    let mandatory_times = [
        std::time::Duration::from_micros(1),
        std::time::Duration::from_millis(1),
        std::time::Duration::from_secs(1),
        std::time::Duration::from_mins(1),
    ];

    let times = mandatory_times.into_iter().chain(std::iter::from_fn(|| {
        Some(std::time::Duration::from_micros(123))
    }));

    let run_return_count = 20;
    let run_returns = solution_parts
        .take(run_return_count)
        .zip(times.take(run_return_count))
        .map(|(solution_part, time_taken)| RunReturn {
            solution_part,
            time_taken,
        });

    let checked_run_returns = run_returns
        .zip(check_returns.clone())
        .map(|(rr, cr)| match cr {
            CheckReturn::Passed => CheckedRunReturn::Ok(rr),
            CheckReturn::Failed(ce) => CheckedRunReturn::CheckFailed(ce),
            CheckReturn::Unchecked(uc) => CheckedRunReturn::Unchecked {
                reason: uc,
                ret: rr,
            },
        });

    let part_ouputs = check_returns
        .map(PartOutput::Checked)
        .chain(checked_run_returns.map(PartOutput::CheckedAndRan));

    let acquisition_errors = [
        AcquisitionError::NotMapped,
        AcquisitionError::NotMapped,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
        AcquisitionError::Unimplemented,
    ];

    part_ouputs
        .map(Ok)
        .chain(acquisition_errors.into_iter().map(Err))
}
