#![windows_subsystem = "windows"]
//crates
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate gtk_source_sys;
extern crate serde;
extern crate serde_json;
extern crate sourceview;
#[macro_use]
extern crate serde_derive;

//mods
mod MarketTransaction_structure;
mod SecuritizationProgram_structure;
mod SecurityPosition_structure;
mod credit_contract_read;
mod credit_contract_structure;
mod crm_read;
mod crm_structure;
mod pool_read;
mod return_pool_structure;
mod MarketTransaction_read;
mod pool_structure;
mod SecurityPosition_read;

//uses
use credit_contract_structure::CreditContract;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    Application, ApplicationWindow, Box, Builder, Button, ComboBoxText, Entry, FileChooserButton,
    Paned,
};
use sourceview::{
    Buffer, BufferExt, LanguageManager, LanguageManagerExt, StyleSchemeExt, StyleSchemeManager,
    StyleSchemeManagerExt, View,
};
use std::fs;
use std::path::PathBuf;
use std::thread;



pub struct Message {
    expos: String,
    crms: String,
}

impl Message {
    pub fn new() -> Message {
        Message {
            expos: "{\"empty\" : \"empty\"}".to_string(),
            crms: "{\"empty\" : \"empty\"}".to_string(),
        }
    }
}

pub enum Structure {
    CreditContract,
}

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("AppGUI2.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    sourceview::View::static_type();
    let primary_panel: Paned = builder
        .get_object("primaryPanel")
        .expect("Could't get primaryPanel");
    let update_search_screen: Box = builder
        .get_object("updateSearchScreen")
        .expect("Could't get updateSearchScreen");
    let language_manager = LanguageManager::new();
    let language = language_manager.get_language("json").unwrap();
    let style_manager = StyleSchemeManager::new();
    let my_style = style_manager.get_scheme("darkmate").unwrap();

    // Call objects
    let pick_folder_call_btn: FileChooserButton = builder
        .get_object("PickFolderCallBtn")
        .expect("Cound't get PickFolderCallBtn");
    let search_call_btn: Button = builder
        .get_object("SearchCallBtn")
        .expect("Cound't get SearchCallBtn");
    let cc_id_search_call_entry: Entry = builder
        .get_object("CcSearchCallEntry")
        .expect("Cound't get IdSearchCallEntry");
    let pool_id_search_call_entry: Entry = builder
        .get_object("PoolSearchCallEntry")
        .expect("Cound't get IdSearchCallEntry");
    let mkt_id_search_call_entry: Entry = builder
        .get_object("MkTSearchCallEntry")
        .expect("Cound't get MkTSearchCallEntry");
    let secpos_id_search_call_entry: Entry = builder
        .get_object("SecPosSearchCallEntry")
        .expect("Cound't get SecPosSearchCallEntry");
    let secprog_id_search_call_entry: Entry = builder
        .get_object("SecProgSearchCallEntry")
        .expect("Cound't get SecProgSearchCallEntry");
    let crm_search_call_entry: Entry = builder
        .get_object("CrmSearchCallEntry")
        .expect("Cound't get CrmSearchCallEntry");
    let credit_contract_call_cb: ComboBoxText = builder
        .get_object("CreditContractCallCb")
        .expect("Cound't get CreditContractCallCb");
    let pool_call_cb: ComboBoxText = builder
        .get_object("PoolCallCb")
        .expect("Cound't get PoolCallCb");
    let crm_call_cb: ComboBoxText = builder
        .get_object("CrmCallCb")
        .expect("Cound't get CrmCallCb");
    let secprog_call_cb: ComboBoxText = builder
        .get_object("SecProgCallCb")
        .expect("Cound't get CrmCallCb");
    let market_transaction_call_cb: ComboBoxText = builder
        .get_object("MarketTransactionCallCb")
        .expect("Cound't get MarketTransactionCallCb");
    let security_position_call_cb: ComboBoxText = builder
        .get_object("SecurityPositionCallCb")
        .expect("Cound't get SecurityPositionCallCb");
    let pool_call_json_expo: View = builder
        .get_object("callPoolExpoJson")
        .expect("Cound't get callPoolExpoJson");
    let pool_call_json_expo_buffer = Buffer::new_with_language(&language);
    pool_call_json_expo_buffer.set_style_scheme(Some(&my_style));
    pool_call_json_expo.set_buffer(Some(&pool_call_json_expo_buffer));
    let cc_call_json_expo: View = builder
        .get_object("callCcExpoJson")
        .expect("Cound't get callCcExpoJson");
    let cc_call_json_expo_buffer = Buffer::new_with_language(&language);
    cc_call_json_expo_buffer.set_style_scheme(Some(&my_style));
    cc_call_json_expo.set_buffer(Some(&cc_call_json_expo_buffer));
    let cc_call_json_crm: View = builder
        .get_object("callCcCrmJson")
        .expect("Cound't get callCcCrmJson");
    let cc_call_json_crm_buffer = Buffer::new_with_language(&language);
    cc_call_json_crm_buffer.set_style_scheme(Some(&my_style));
    cc_call_json_crm.set_buffer(Some(&cc_call_json_crm_buffer));
    let mkt_call_json_expo: View = builder
        .get_object("callMarketTransactionExpoJson")
        .expect("Cound't get callMarketTransactionExpoJson");
    let mkt_call_json_expo_buffer = Buffer::new_with_language(&language);
    mkt_call_json_expo_buffer.set_style_scheme(Some(&my_style));
    mkt_call_json_expo.set_buffer(Some(&mkt_call_json_expo_buffer));
    let mkt_call_json_crm: View = builder
        .get_object("callMarketTransactionCrmJson")
        .expect("Cound't get callMarketTransactionCrmJson");
    let mkt_call_json_crm_buffer = Buffer::new_with_language(&language);
    mkt_call_json_crm_buffer.set_style_scheme(Some(&my_style));
    mkt_call_json_crm.set_buffer(Some(&mkt_call_json_crm_buffer));
    let ps_call_json_expo: View = builder
        .get_object("callSecurityPositionExpoJson")
        .expect("Cound't get callSecurityPositionExpoJson");
    let ps_call_json_expo_buffer = Buffer::new_with_language(&language);
    ps_call_json_expo_buffer.set_style_scheme(Some(&my_style));
    ps_call_json_expo.set_buffer(Some(&mkt_call_json_expo_buffer));
    let ps_call_json_crm: View = builder
        .get_object("callSecurityPositionCrmJson")
        .expect("Cound't get callSecurityPositionCrmJson");
    let ps_call_json_crm_buffer = Buffer::new_with_language(&language);
    ps_call_json_crm_buffer.set_style_scheme(Some(&my_style));
    ps_call_json_crm.set_buffer(Some(&mkt_call_json_crm_buffer));
    let crm_call_json: View = builder
        .get_object("callCrmJson")
        .expect("Cound't get callCrmJson");
    let crm_call_json_buffer = Buffer::new_with_language(&language);
    crm_call_json_buffer.set_style_scheme(Some(&my_style));
    crm_call_json.set_buffer(Some(&crm_call_json_buffer));
    let securitisation_program_call_expo_json: View = builder
        .get_object("callSecuritizationProgramExpoJson")
        .expect("Cound't get callSecuritizationProgramExpoJson");
    let securitisation_program_call_expo_json_buffer = Buffer::new_with_language(&language);
    securitisation_program_call_expo_json_buffer.set_style_scheme(Some(&my_style));
    securitisation_program_call_expo_json
        .set_buffer(Some(&securitisation_program_call_expo_json_buffer));
    let securitisation_program_call_crm_json: View = builder
        .get_object("callSecuritizationProgramCrmJson")
        .expect("Cound't get callSecuritizationProgramExpoJson");
    let securitisation_program_call_crm_json_buffer = Buffer::new_with_language(&language);
    securitisation_program_call_crm_json_buffer.set_style_scheme(Some(&my_style));
    securitisation_program_call_crm_json
        .set_buffer(Some(&securitisation_program_call_crm_json_buffer));

    //Return Object
    let pick_folder_return_btn: FileChooserButton = builder
        .get_object("PickFolderReturnBtn")
        .expect("Cound't get PickFolderReturnBtn");
    let search_return_btn: Button = builder
        .get_object("SearchReturnBtn")
        .expect("Cound't get SearchReturnBtn");
    let cc_id_search_return_entry: Entry = builder
        .get_object("CcSearchReturnEntry")
        .expect("Cound't get CcSearchReturnEntry");
    let pool_id_search_return_entry: Entry = builder
        .get_object("PoolSearchReturnEntry")
        .expect("Cound't get PoolSearchReturnEntry");
    let mkt_id_search_return_entry: Entry = builder
        .get_object("MkTSearchReturnEntry")
        .expect("Cound't get MkTSearchReturnEntry");
    let secpos_id_search_return_entry: Entry = builder
        .get_object("SecPosSearchReturnEntry")
        .expect("Cound't get SecPosSearchReturnEntry");
    let secprog_id_search_return_entry: Entry = builder
        .get_object("SecProgSearchReturnEntry")
        .expect("Cound't get SecProgSearchReturnEntry");
    let crm_search_return_entry: Entry = builder
        .get_object("CrmSearchReturnEntry")
        .expect("Cound't get CrmSearchReturnEntry");
    let credit_contract_return_cb: ComboBoxText = builder
        .get_object("CreditContractCallCb")
        .expect("Cound't get CreditContractCallCb");
    let credit_contract_return_cb: ComboBoxText = builder
        .get_object("CreditContractReturnCb")
        .expect("Cound't get CreditContractReturnCb");
    let pool_return_cb: ComboBoxText = builder
        .get_object("PoolReturnCb")
        .expect("Cound't get PoolReturnCb");
    let crm_return_cb: ComboBoxText = builder
        .get_object("CrmReturnCb")
        .expect("Cound't get CrmReturnCb");
    let market_transaction_return_cb: ComboBoxText = builder
        .get_object("MarketTransactionReturnCb")
        .expect("Cound't get MarketTransactionReturnCb");
    let security_position_return_cb: ComboBoxText = builder
        .get_object("SecurityPositionReturnCb")
        .expect("Cound't get SecurityPositionReturnCb");
    let pool_return_json_expo: View = builder
        .get_object("returnPoolExpoJson")
        .expect("Cound't get returnPoolExpoJson");
    let pool_return_json_expo_buffer = Buffer::new_with_language(&language);
    pool_return_json_expo_buffer.set_style_scheme(Some(&my_style));
    pool_return_json_expo.set_buffer(Some(&pool_return_json_expo_buffer));
    let cc_return_json_expo: View = builder
        .get_object("returnCcExpoJson")
        .expect("Cound't get callCcExpoJson");
    let cc_return_json_expo_buffer = Buffer::new_with_language(&language);
    cc_return_json_expo_buffer.set_style_scheme(Some(&my_style));
    cc_return_json_expo.set_buffer(Some(&cc_return_json_expo_buffer));
    let cc_return_json_crm: View = builder
        .get_object("returnCcCrmJson")
        .expect("Cound't get returnCcCrmJson");
    let cc_return_json_crm_buffer = Buffer::new_with_language(&language);
    cc_return_json_crm_buffer.set_style_scheme(Some(&my_style));
    cc_return_json_crm.set_buffer(Some(&cc_return_json_crm_buffer));
    let mt_return_json_expo: View = builder
        .get_object("returnMarketTransactionExpoJson")
        .expect("Cound't get returnMarketTransactionExpoJson");
    let mt_return_json_expo_buffer = Buffer::new_with_language(&language);
    mt_return_json_expo_buffer.set_style_scheme(Some(&my_style));
    mt_return_json_expo.set_buffer(Some(&mt_return_json_expo_buffer));
    let mt_return_json_crm: View = builder
        .get_object("returnMarketTransactionCrmJson")
        .expect("Cound't get returnMarketTransactionCrmJson");
    let mt_return_json_crm_buffer = Buffer::new_with_language(&language);
    mt_return_json_crm_buffer.set_style_scheme(Some(&my_style));
    mt_return_json_crm.set_buffer(Some(&mt_return_json_crm_buffer));
    let ps_return_json_expo: View = builder
        .get_object("returnSecurityPositionExpoJson")
        .expect("Cound't get returnSecurityPositionExpoJson");
    let ps_return_json_expo_buffer = Buffer::new_with_language(&language);
    ps_return_json_expo_buffer.set_style_scheme(Some(&my_style));
    ps_return_json_expo.set_buffer(Some(&mt_return_json_expo_buffer));
    let ps_return_json_crm: View = builder
        .get_object("returnSecurityPositionCrmJson")
        .expect("Cound't get returnSecurityPositionCrmJson");
    let ps_return_json_crm_buffer = Buffer::new_with_language(&language);
    ps_return_json_crm_buffer.set_style_scheme(Some(&my_style));
    ps_return_json_crm.set_buffer(Some(&mt_return_json_crm_buffer));
    let crm_return_json: View = builder
        .get_object("returnCrmJson")
        .expect("Cound't get returnCrmJson");
    let crm_return_json_buffer = Buffer::new_with_language(&language);
    crm_return_json_buffer.set_style_scheme(Some(&my_style));
    crm_return_json.set_buffer(Some(&crm_return_json_buffer));
    let securitisation_program_return_expo_json: View = builder
        .get_object("returnSecuritizationProgramExpoJson")
        .expect("Cound't get returnSecuritizationProgramExpoJson");
    let securitisation_program_return_expo_json_buffer = Buffer::new_with_language(&language);
    securitisation_program_return_expo_json_buffer.set_style_scheme(Some(&my_style));
    securitisation_program_return_expo_json
        .set_buffer(Some(&securitisation_program_return_expo_json_buffer));
    let securitisation_program_return_crm_json: View = builder
        .get_object("returnSecuritizationProgramCrmJson")
        .expect("Cound't get returnSecuritizationProgramCrmJson");
    let securitisation_program_return_crm_json_buffer = Buffer::new_with_language(&language);
    securitisation_program_return_crm_json_buffer.set_style_scheme(Some(&my_style));
    securitisation_program_return_crm_json
        .set_buffer(Some(&securitisation_program_return_crm_json_buffer));

    //behaviours
    pick_folder_call_btn.connect_file_set(clone!(
    @weak  pool_call_cb,
    @weak credit_contract_call_cb,
    @weak crm_call_cb,
    @weak market_transaction_call_cb,
    @weak security_position_call_cb
    => move |pick_folder_btn| {
        let tmp : PathBuf = pick_folder_btn.get_filename().unwrap();
        let my_str: String = tmp.into_os_string().into_string().unwrap();
        let mut v: Vec<String> = Vec::new();
        let files = fs::read_dir(my_str).unwrap();
        files
            .filter_map(Result::ok)
            .filter(|d| if let Some(e) = d.path().extension() { e == "json" } else {false})
            .for_each(|f| v.push(f.file_name().into_string().unwrap()));
            for f in v.iter() {
                if f == "creditContracts.json" {
                    credit_contract_call_cb.prepend_text(&f);
                    credit_contract_call_cb.set_active(Some(0));
                } else {
                    credit_contract_call_cb.append_text(&f);
                };
                if f == "pools.json" {
                    pool_call_cb.prepend_text(&f);
                    pool_call_cb.set_active(Some(0));
                } else {
                    pool_call_cb.append_text(&f);
                };
                if f == "crms.json" {
                    crm_call_cb.prepend_text(&f);
                    crm_call_cb.set_active(Some(0));
                } else {
                    crm_call_cb.append_text(&f);
                };
                if f == "marketTransactions.json" {
                    market_transaction_call_cb.prepend_text(&f);
                    market_transaction_call_cb.set_active(Some(0));
                } else {
                    market_transaction_call_cb.append_text(&f);
                };
                if f == "securityPositions.json" {
                    security_position_call_cb.prepend_text(&f);
                    security_position_call_cb.set_active(Some(0));
                } else {
                    security_position_call_cb.append_text(&f);
                };
            }
        }));

    search_call_btn.connect_clicked(
        clone!( 
        @weak pool_call_json_expo,
        @weak cc_call_json_expo,
        @weak cc_call_json_crm,
        @weak mkt_call_json_expo,
        @weak mkt_call_json_crm,
        @weak ps_call_json_expo,
        @weak ps_call_json_crm,
        @weak update_search_screen ,
        @weak primary_panel
            => move |_|{
        //primary_panel.hide();
        //update_search_screen.show();
        let tmp : PathBuf = pick_folder_call_btn.get_filename().unwrap();
        let folder: String = tmp.into_os_string().into_string().unwrap();
        let idsearchvcc: String = cc_id_search_call_entry.get_text().to_string();
        let idsearchvpool: String = pool_id_search_call_entry.get_text().to_string();
        let idsearchvmkt: String = mkt_id_search_call_entry.get_text().to_string();
        let idsearchvsecpos: String = secpos_id_search_call_entry.get_text().to_string();
        let idsearchvsecprog: String = secprog_id_search_call_entry.get_text().to_string();
        let idcrmv: String = crm_search_call_entry.get_text().to_string().split(";").collect();
        let creditcontracttmp = credit_contract_call_cb.get_active_text().unwrap().to_string();
        let creditcontractv= format!(r"{}\{}", folder, creditcontracttmp);
        let crmtmp = crm_call_cb.get_active_text().unwrap().to_string();
        let crmv= format!(r"{}\{}", folder, crmtmp);
        let pooltmp  = pool_call_cb.get_active_text().unwrap().to_string();
        let poolv= format!(r"{}\{}", folder, pooltmp);
        let markettransactmp = market_transaction_call_cb.get_active_text().unwrap().to_string();
        let markettransactv= format!(r"{}\{}", folder, markettransactmp);
        let securitypostmp = security_position_call_cb.get_active_text().unwrap().to_string();
        let securityposv= format!(r"{}\{}", folder, securitypostmp);
        let crm_ccv= crmv.to_owned();
        let (sender_pool, receiver_pool) = glib::MainContext::sync_channel(glib::PRIORITY_DEFAULT, 4000);
            

        thread::spawn(move || {
        if idsearchvpool != "".to_string() && poolv !="".to_string() {
            let pool_call_data_tmp = pool_read::pool_load(&idsearchvpool ,&poolv);
            let mut pool_call_data = String::new();
            for elem in pool_call_data_tmp.iter() {
            let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
            &pool_call_data.push_str(&ser_elem);            
        }
        let _ = sender_pool.send(String::from(&pool_call_data));
        }
        });
        receiver_pool.attach(None, move |msg| {
            match msg {
                text => pool_call_json_expo.get_buffer().unwrap().set_text(&text),
            }
            glib::Continue(true)
        });
        
        let (sender_cc, receiver_cc) = glib::MainContext::sync_channel(glib::PRIORITY_DEFAULT, 4000);
        

        thread::spawn(move || {
        if idsearchvcc != "".to_string() && creditcontractv !="".to_string() {
            let mut thread_message = Message::new();
            let cc_call_data_tmp = credit_contract_read::cc_load(&idsearchvcc ,&creditcontractv);
            let mut cc_call_data = String::new();
            for elem in cc_call_data_tmp.iter() {
                let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                cc_call_data.push_str(&ser_elem);
        }

        let cc_struct = match cc_call_data_tmp {
            Ok(contracts) => contracts,
            Err(error) => panic!("Problem occured: {:?}", error)
        };

        let mut cc_crm_call_data = String::new();
        let param = cc_struct.len();
            if param != 0  {
            let crm_result = crm_read::crm_cc(&cc_struct);
            let crm_list = match crm_result {
                Ok(crm) => crm,
                Err(error) => panic!("Problem occured: {:?}", error)
            };
            
            let cc_crm_call_data_tmp = crm_read::crm_load(&crm_list ,&crm_ccv);
                for elem in cc_crm_call_data_tmp.iter() {
                    let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                    cc_crm_call_data.push_str(&ser_elem);
                }
        }
        
        thread_message.crms = cc_crm_call_data;
        thread_message.expos = cc_call_data;

        let _ = sender_cc.send(thread_message);
    }
        });
        receiver_cc.attach(None, move |msg| {
            match msg.expos {
                text => cc_call_json_expo.get_buffer().unwrap().set_text(&text),
            }
            match msg.crms {
                text => cc_call_json_crm.get_buffer().unwrap().set_text(&text),
            }

            glib::Continue(true)
        });
        
        let (sender_mkt, receiver_mkt) = glib::MainContext::sync_channel(glib::PRIORITY_DEFAULT, 4000);
        let crm_mkt= crmv.to_owned();

        thread::spawn(move || {
            if idsearchvmkt != "".to_string() && markettransactv !="".to_string() {
                let mut thread_message = Message::new();
                let mkt_call_data_tmp = MarketTransaction_read::mkt_load(&idsearchvmkt ,&markettransactv);
                let mut mkt_call_data = String::new();
                for elem in mkt_call_data_tmp.iter() {
                    let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                    mkt_call_data.push_str(&ser_elem);
            }
    
            let mkt_struct = match mkt_call_data_tmp {
                Ok(contracts) => contracts,
                Err(error) => panic!("Problem occured: {:?}", error)
            };
    
            let mut mkt_crm_call_data = String::new();
            let param = mkt_struct.len();
            if param != 0 {
                let crm_result = crm_read::crm_mkt(&mkt_struct);
                let crm_list = match crm_result {
                    Ok(crm) => crm,
                    Err(error) => panic!("Problem occured: {:?}", error)
                };
                
                let mkt_crm_call_data_tmp = crm_read::crm_load(&crm_list ,&crm_mkt);
                    for elem in mkt_crm_call_data_tmp.iter() {
                        let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                        mkt_crm_call_data.push_str(&ser_elem);
                    }
            
            }
            
            thread_message.crms = mkt_crm_call_data;
            thread_message.expos = mkt_call_data;
    
            let _ = sender_mkt.send(thread_message);
        }
            });
            receiver_mkt.attach(None, move |msg| {
                match msg.expos {
                    text => mkt_call_json_expo.get_buffer().unwrap().set_text(&text),
                }
                match msg.crms {
                    text => mkt_call_json_crm.get_buffer().unwrap().set_text(&text),
                }
    
                glib::Continue(true)
            });

        let (sender_pos_sec, receiver_pos_sec) = glib::MainContext::sync_channel(glib::PRIORITY_DEFAULT, 4000);
        let crm_pos_sec= crmv.to_owned();

        thread::spawn(move || {
            if idsearchvsecpos != "".to_string() && securityposv !="".to_string() {
                let mut thread_message = Message::new();
                let pos_sec_call_data_tmp = SecurityPosition_read::secpos_load(&idsearchvsecpos ,&securityposv);
                let mut pos_sec_call_data = String::new();
                for elem in pos_sec_call_data_tmp.iter() {
                    let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                    pos_sec_call_data.push_str(&ser_elem);
            }
    
            let pos_sec_struct = match pos_sec_call_data_tmp {
                Ok(contracts) => contracts,
                Err(error) => panic!("Problem occured: {:?}", error)
            };
    
            let mut pos_sec_crm_call_data = String::new();
            let param = pos_sec_struct.len();
            if param != 0 {
                let crm_result = crm_read::crm_pos_sec(&pos_sec_struct);
                let crm_list = match crm_result {
                    Ok(crm) => crm,
                    Err(error) => panic!("Problem occured: {:?}", error)
                };
                
                let pos_sec_crm_call_data_tmp = crm_read::crm_load(&crm_list ,&crm_pos_sec);
                    for elem in pos_sec_crm_call_data_tmp.iter() {
                        let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
                        pos_sec_crm_call_data.push_str(&ser_elem);
                    }
            
            }
            
            thread_message.crms = pos_sec_crm_call_data;
            thread_message.expos = pos_sec_call_data;
    
            let _ = sender_pos_sec.send(thread_message);
        }
            });
            receiver_pos_sec.attach(None, move |msg| {
                match msg.expos {
                    text => ps_call_json_expo.get_buffer().unwrap().set_text(&text),
                }
                match msg.crms {
                    text => ps_call_json_crm.get_buffer().unwrap().set_text(&text),
                }
    
                glib::Continue(true)
            });

        })            
    );

    pick_folder_return_btn.connect_file_set(clone!(
    @weak  pool_return_cb,
    @weak credit_contract_return_cb,
    @weak crm_return_cb,
    @weak market_transaction_return_cb,
    @weak security_position_return_cb
    => move |pick_folder_btn| {
        let tmp : PathBuf = pick_folder_btn.get_filename().unwrap();
        let my_str: String = tmp.into_os_string().into_string().unwrap();
        let mut v: Vec<String> = Vec::new();
        let files = fs::read_dir(my_str).unwrap();
        files
            .filter_map(Result::ok)
            .filter(|d| if let Some(e) = d.path().extension() { e == "json" } else {false})
            .for_each(|f| v.push(f.file_name().into_string().unwrap()));
            for f in v.iter() {
                if f == "creditContracts.json" {
                    credit_contract_return_cb.prepend_text(&f);
                    credit_contract_return_cb.set_active(Some(0));
                } else {
                    credit_contract_return_cb.append_text(&f);
                };
                if f == "pools.json" {
                    pool_return_cb.prepend_text(&f);
                    pool_return_cb.set_active(Some(0));
                } else {
                    pool_return_cb.append_text(&f);
                };
                if f == "crms.json" {
                    crm_return_cb.prepend_text(&f);
                    crm_return_cb.set_active(Some(0));
                } else {
                    crm_return_cb.append_text(&f);
                };
                if f == "marketTransactions.json" {
                    market_transaction_return_cb.prepend_text(&f);
                    market_transaction_return_cb.set_active(Some(0));
                } else {
                    market_transaction_return_cb.append_text(&f);
                };
                if f == "securityPositions.json" {
                    security_position_return_cb.prepend_text(&f);
                    security_position_return_cb.set_active(Some(0));
                } else {
                    security_position_return_cb.append_text(&f);
                };
            }
        }));

    search_return_btn.connect_clicked(clone!(
    @weak pool_return_json_expo,
    @weak update_search_screen ,
    @weak primary_panel,
    @weak cc_return_json_expo
    => move |_|{
    primary_panel.hide();
    update_search_screen.show();
    let tmp : PathBuf = pick_folder_return_btn.get_filename().unwrap();
    let folder: String = tmp.into_os_string().into_string().unwrap();
    let idsearchvcc: String = cc_id_search_return_entry.get_text().to_string();
    let idsearchvpool: String = pool_id_search_return_entry.get_text().to_string();
    let idsearchvmkt: String = mkt_id_search_return_entry.get_text().to_string();
    let idsearchvsecpos: String = secpos_id_search_return_entry.get_text().to_string();
    let idsearchvsecprog: String = secprog_id_search_return_entry.get_text().to_string();
    let idcrmv: String = crm_search_return_entry.get_text().to_string().split(";").collect();
    let creditcontracttmp = credit_contract_return_cb.get_active_text().unwrap().to_string();
    let creditcontractv= format!(r"{}\{}", folder, creditcontracttmp);
    let pooltmp  = pool_return_cb.get_active_text().unwrap().to_string();
    let poolv= format!(r"{}\{}", folder, pooltmp);
    let markettransactmp = market_transaction_return_cb.get_active_text().unwrap().to_string();
    let markettransactv= format!(r"{}\{}", folder, markettransactmp);
    let securitypostmp = security_position_return_cb.get_active_text().unwrap().to_string();
    let securityposv= format!(r"{}\{}", folder, securitypostmp);

    let pool_return_data_tmp = pool_read::return_pool_load(&idsearchvpool ,&poolv);
    let mut return_pool_data = String::new();
    for elem in pool_return_data_tmp.iter() {
        let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
        return_pool_data.push_str(&ser_elem);
    }
    pool_return_json_expo.get_buffer().unwrap().set_text(&return_pool_data);

    let cc_data_tmp = credit_contract_read::cc_load(&idsearchvcc ,&creditcontractv);
    let mut cc_data = String::new();
    for elem in cc_data_tmp.iter() {
        let ser_elem = serde_json::to_string_pretty(&elem).unwrap();
        cc_data.push_str(&ser_elem);
    }
    cc_return_json_expo.get_buffer().unwrap().set_text(&cc_data);

    update_search_screen.hide();
    primary_panel.show();
    }));

    window.show_all();
    update_search_screen.hide();
}

fn main() {
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        build_ui(app);
    });
    application.run(&[]);
}
