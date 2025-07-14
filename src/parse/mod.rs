use silphium::ModuleMap;

mod manifest;
pub use manifest::Manifest;

mod strings {
    use pest_derive::Parser as Pest;

    #[derive(Pest)]
    #[grammar = "strings.pest"]
    pub struct Parser;
}

mod descr_mercenaries {
    use pest_derive::Parser as Pest;

    #[derive(Pest)]
    #[grammar = "descr_mercenaries.pest"]
    pub struct Parser;
}

mod descr_strat {
    use pest_derive::Parser as Pest;

    #[derive(Pest)]
    #[grammar = "descr_strat.pest"]
    pub struct Parser;
}

mod export_descr_buildings {
    use pest_derive::Parser as Pest;

    #[derive(Pest)]
    #[grammar = "export_descr_buildings.pest"]
    pub struct Parser;
}

mod descr_sm_factions {
    mod og {
        use pest_derive::Parser as Pest;

        #[derive(Pest)]
        #[grammar = "descr_sm_factions.og.pest"]
        pub struct Parser;
    }
    mod rr {
        use pest_derive::Parser as Pest;

        #[derive(Pest)]
        #[grammar = "descr_sm_factions.rr.pest"]
        pub struct Parser;
    }
}

pub fn parse_folder(_manifest: Manifest) -> ModuleMap {
    todo!()
}
