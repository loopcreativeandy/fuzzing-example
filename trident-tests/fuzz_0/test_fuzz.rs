use fuzzme::entry as entry_fuzzme;
use fuzzme::ID as PROGRAM_ID_FUZZME;
const PROGRAM_NAME_FUZZME: &str =  "fuzzme";
use fuzz_instructions::fuzzme_fuzz_instructions::FuzzInstruction as FuzzInstruction_fuzzme;
use trident_client::fuzzing::*;
mod accounts_snapshots;
mod fuzz_instructions;

pub type FuzzInstruction = FuzzInstruction_fuzzme;

struct MyFuzzData;

impl FuzzDataBuilder<FuzzInstruction> for MyFuzzData {}

fn main() {
    loop {
        fuzz_trident!(fuzz_ix: FuzzInstruction, |fuzz_data: MyFuzzData| {

            // Specify programs you want to include in genesis
            // Programs without an `entry_fn`` will be searched for within `trident-genesis` folder.
            // `entry_fn`` example: processor!(convert_entry!(program_entry))
            let fuzzing_program1 = FuzzingProgram::new(PROGRAM_NAME_FUZZME,&PROGRAM_ID_FUZZME,processor!(convert_entry!(entry_fuzzme)));

            let mut client =
                ProgramTestClientBlocking::new(&[fuzzing_program1])
                    .unwrap();

            // fill Program ID of program you are going to call
            let _ = fuzz_data.run_with_runtime(PROGRAM_ID_FUZZME, &mut client);
        });
    }
}
