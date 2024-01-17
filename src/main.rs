slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_utregning_belop(move |total, rente, avdrag, strom1, strom2, strom3| {
        let ui = ui_handle.unwrap();
        let total_num = total.trim().parse::<f64>().unwrap_or(0.0);
        let rente_num = rente.trim().parse::<f64>().unwrap_or(0.0);
        let avdrag_num = avdrag.trim().parse::<f64>().unwrap_or(0.0);
        let strom1_num = strom1.trim().parse::<f64>().unwrap_or(0.0);
        let strom2_num = strom2.trim().parse::<f64>().unwrap_or(0.0);
        let strom3_num = strom3.trim().parse::<f64>().unwrap_or(0.0);

        let rente2_num: f64 = rente_num / 2.0;
        let mari_andel: f64 = total_num - rente_num;
        let totalen_andeler: f64 = avdrag_num + mari_andel;
        let andel_eirik: f64 = avdrag_num / totalen_andeler;
        let sum_eirik: f64 = mari_andel * andel_eirik + rente2_num + strom1_num + strom2_num + strom3_num;
        let result = format!("Eirik skal betale: {:.2} ", sum_eirik);

        ui.set_results(result.into());
    });

    ui.run()
}
