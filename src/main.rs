slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_utregning_belop(move |total, rente, avdrag, strom1, strom2, strom3| {
        let ui = ui_handle.unwrap();

        let total_num = parse_to_f64(&total);
        let rente_num = parse_to_f64(&rente);
        let avdrag_num = parse_to_f64(&avdrag);
        let strom1_num = parse_to_f64(&strom1) / 2.0;
        let strom2_num = parse_to_f64(&strom2) / 2.0;
        let strom3_num = parse_to_f64(&strom3) / 2.0;

        let rente2_num = rente_num / 2.0;
        let mari_andel = total_num - rente_num;
        let totalen_andeler = avdrag_num + mari_andel;
        let andel_eirik = avdrag_num / totalen_andeler;
        let sum_eirik = mari_andel * andel_eirik + rente2_num + strom1_num + strom2_num + strom3_num;

        let result = format!("Eirik skal betale: {:.2} ", sum_eirik);
        ui.set_results(result.into());
    });

    ui.run()
}

fn parse_to_f64(input: &str) -> f64 {
    input.trim().replace(',', ".").parse::<f64>().unwrap_or(0.0)
}
