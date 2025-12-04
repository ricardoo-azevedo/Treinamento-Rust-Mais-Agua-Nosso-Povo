use std::{collections::BTreeMap, io::{self, Write}};

use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Clone)]
struct Familia {
    id: u32,
    nis: String,
    endereco: String,
    possui_captaca_calhas: bool,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Clone)]
struct Membro {
    id: u32,
    cpf: String,
    nome: String,
    ano_nasc: i32,
    acamado: bool,
    id_familia: u32
}

#[derive(Debug, Clone)]
struct Cisterna {
    id: u32,
    capacidade_litros: i64,
    nivel_atual: i64,
    id_familia: u32
}

#[derive(Debug, Clone)]
struct Distribuicao {
    id: u32,
    data_entrega: NaiveDate,
    quantidade_litros: i64,
    previsao_proxima: NaiveDate,
    observacoes: String,
    criacao_local_date_time: NaiveDateTime,
    id_cisterna: u32,
    id_familia: u32
}

fn input(p: &str) -> io::Result<String>{
    print!("{p}");
    io::stdout().flush()?;
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf.trim().to_string())
}

fn gerar_id<T>(lista: &BTreeMap<u32, T>) -> u32 {
    lista.keys().last().map(|x| x + 1).unwrap_or(1)
}

fn cadastrar_familia (lista_familia: &mut BTreeMap<u32, Familia>) -> Result<(), Box<dyn std::error::Error>> {
    println!("CADASTRO DE FAMILIA"); 

    let id  = gerar_id(lista_familia);
    
    let nis = input("Código NIS da familia: ")?;

    let endereco = input("Endereço: ")?;

    let r = input("Possui captação calhas: [y/n]")?;

    let cap =  match r.trim() {
        "y" | "Y" | "s" | "S" => true,
        "n" | "N" => false,
        _ => {
            println!("Valor inválido, considerando como 'não'.");
            false
        }
    };

    let latitude: f64 = input("Informe a latitude do endereço: ")?.trim().parse()?;

    let longitude: f64 = input("Informe a longitude do endereço: ")?.trim().parse()?;

    lista_familia.insert(
        id, 
        Familia {
            id, 
            endereco,
            nis,
            possui_captaca_calhas: cap,
            latitude,
            longitude
        }
    );
    Ok(())
}
fn main() {
}
