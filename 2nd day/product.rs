// // struct product{
// //     name : string,
// //     price : float,
// //     category : string,
// // }
// // pub fn __init__(name,price,category)
// // {
// //     let mut name=string::new();
// //     let mut price=float::new();
// //     let mut category=string::new();
// //    let mut name=string::new();io::stdin().read_line(&mut name).expect("failed to read input");
// //     let mut price = float::new();io::stdin().read_line(&mut price).expect("failed to read input");
// //    let mut  category = int::new();io::stdin().read_line(&mut category).expect("failed to read input");
// // }
// impl product{
//     pub fn __init__(args: &product) {

//         let mut name=string::new();
//         let mut price=float::new();
//         let mut category=string::new();
//     }
// }
// pub fn print_self()
// {
//     println!("{} {} {}",name,price,category)
// }
// pub fn get_last_id()
// {
//     product = file::read("data/product.csv").expect("file not found");
//     if products.len() == 0
//     {
//         return 0;
//     }
//     else
//     {
//         return products[products.len()-1].id;
//     }
// }



struct Product {
    name : string, 
    price : float, 
    category : string,
    stock : int,
}

// fn __init__(args: &Product, name:string, price : float, category: string, stock:int){
//     args.name = name;
//     args.price = price;
//     args.category = category;
//     args.stock = stock;
// }

impl Product {
    fn __init__(name: string, price:float, category:string, stock:int) -> Self {
        Self { name, price, category, stock}
    }
    

    fn get_all_products(){
        file::read()
    }


}

