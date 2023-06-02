//TRASFERITO LA GESTIONE DELL'ESECUZIONE SU UNA CLASSE HANDLER, MOLTO PIÙ FACILE IL PASSAGGIO DELLE VARIABILI per fortuna
//cambiato gestione input per gestire le eccezioni Err
//corretta la gestione della scelta con le tempistiche



fn main() {
    let mut handler=Handler::new();
    handler.start()
    
}

/*  Write a Rust program to manage a fleet of emergency vehicles.

Each vehicle has its own:

    unique alphanumeric id
    position: a pair of values as coordinates on a cartesian plane
    level of equipment, to manage emergencies of corresponding gravity; each emergency is classified in one of the following levels: white, green, yellow, red (more serious)
    availability: a vehicle can be already assigned to an emergency, or it may be available

Suppose that in the fleet there are only two types of vehicles: ambulances and helicopters.

    Ambulances can start immediately and reach the assigned point at an average speed of 100 km/h, but only moving along horizontal and vertical lines (Manhattan distance).
    Helicopters require a setup phase of 5 minutes, before takeoff; then they travel in beeline (straight line) at an average speed of 250 km/h.

The program allows its user to:

    Show data of all vehicles in the fleet
    Add a vehicle together with its data (id, position, level of equipment, availability)
    Assign the quickest available vehicle, with sufficient equipment, to manage a new emergency in a given location

Use runtime polymorphism to manage abstractly the different kinds of vehicles. */
pub struct Emergency{
    position: (f64,f64),
    level: i32,
}
impl Emergency{
    fn new(position: (f64,f64), level: i32)->Emergency{
        Emergency{
            position,
            level,
        }
    }
    fn get_level(&self)->i32{
        self.level
    }
    fn get_position(&self)->(f64,f64){
        self.position
    }
}

pub struct Handler{
    vehicles: Vec<Box<dyn EmergencyVehicle>>,
    emergencies: Vec<Emergency>,
}
impl Handler{
    fn new()->Handler{
        Handler{
            vehicles: Vec::new(),
            emergencies: Vec::new(),
        }
    }

    fn start(&mut self){
        self.vehicles.push(Box::new(Ambulance{
            id: String::from("A1"),
            position: (0.0,0.0),
            level: 1,
            availability: true,
        }));
        self.vehicles.push(Box::new(Ambulance{
            id: String::from("A2"),
            position: (0.0,0.0),
            level: 2,
            availability: true,
        }));
        self.vehicles.push(Box::new(Helicopter{
            id: String::from("H1"),
            position: (0.0,0.0),
            level: 3,
            availability: true,
        }));
        loop{
            match self.choice(){
                1=>self.add_vehicle(),
                2=>self.new_emergency(),
                3=>self.show_vehicles(),
                4=>break,
                _=>println!("Invalid choice!")
            }
        }
    }

    fn choice(&mut self)->u32{
        println!("1. Add a new vehicle");
        println!("2. Assign a vehicle to an emergency");
        println!("3. Show all vehicles");
        println!("4. Exit");
        let mut choice = String::new();
        match std::io::stdin().read_line(&mut choice){
            Ok(_)=>{},
            Err(_)=>{println!("Error!"); return self.choice();
            },
        }
        let choice:u32= match choice.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.choice();
            },
        };
        choice
    }

    fn add_vehicle(&mut self){
        println!("Insert the type of vehicle: ");
        println!("1. Ambulance");
        println!("2. Helicopter");
        let mut choice = String::new();
        match std::io::stdin().read_line(&mut choice){
            Ok(_)=>{},
            Err(_)=>{println!("Error!"); return self.add_vehicle();
            },
        }
        let choice:i32= match choice.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_vehicle();
            },
        };
        
        match choice{
            1=>self.add_ambulance(),
            2=>self.add_helicopter(),
            _=>println!("Invalid choice!")
        }
    }

    fn new_emergency(&mut self){
        println!("Insert type of emergency");
        println!("1. White");
        println!("2. Green");
        println!("3. Yellow");
        println!("4. Red");
        let mut choice = String::new();
        match std::io::stdin().read_line(&mut choice){
            Ok(_)=>{},
            Err(_)=>{println!("Error!"); return self.new_emergency();
            },
        }
        let choice: i32 = choice.trim().parse().expect("Please type a number!");
        println!("Insert the x position of the emergency: ");
        let mut x = String::new();
        match std::io::stdin().read_line(&mut x){
            Ok(_)=>{},
            Err(_)=>{println!("Error!"); return self.new_emergency();
            },
        }
        let x = match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.new_emergency();
            },
        };
        println!("Insert the y position of the emergency: ");
        let mut y = String::new();
        match std::io::stdin().read_line(&mut y){
            Ok(_)=>{},
            Err(_)=>{println!("Error!"); return self.new_emergency();
            },
        }
        let y = match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.new_emergency();
            },
        };
        self.emergencies.push(Emergency::new((x,y),choice));
        self.assign_vehicle(self.emergencies.len()-1);
    }

    fn assign_vehicle(&mut self,id:usize){
        let emergency=&self.emergencies[id];
        let mut vehiclefree= Vec::new();
        for vehicle in self.vehicles.iter_mut(){
            if vehicle.get_availability() && vehicle.get_level()>=emergency.get_level(){
                vehiclefree.push(vehicle);
            }
        }
        if vehiclefree.len()==0{
            println!("No vehicle available!");
            return;
        }
        
        let mut min=vehiclefree[0].get_time(emergency.get_position());
        let mut index=0;
        for i in 0..vehiclefree.len(){
            if vehiclefree[i].get_time(emergency.get_position())<min{
                min=vehiclefree[i].get_time(emergency.get_position());
                index=i;
            }
        }
        
        println!("The vehicle with id {} is the closest to the emergency with time {}", vehiclefree[index].get_id(), vehiclefree[index].get_time(emergency.get_position()));
        vehiclefree[index].set_availability(false);
        vehiclefree[index].move_to(emergency.get_position());
        
    
        
    }

    fn add_ambulance(&mut self){
        println!("Insert the id of the ambulance: ");
        let mut id = String::new();
        match std::io::stdin().read_line(&mut id){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_ambulance();},
        }
        
        for vehicle in self.vehicles.iter(){
            if vehicle.get_id() == id.trim(){
                println!("An ambulance with this id already exists!, try again:");
                
                return self.add_ambulance();
            }
        }
        println!("Insert the x position of the ambulance: ");
        let mut x = String::new();
        match std::io::stdin().read_line(&mut x){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_ambulance();},
        }
        let x = match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_ambulance();
            },
        };
        println!("Insert the y position of the ambulance: ");
        let mut y: String = String::new();
        match std::io::stdin().read_line(&mut y){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_ambulance();},
        }
        let y = match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_ambulance();
            },
        };
        println!("Insert the level of the ambulance: ");
        println!("1. White");
        println!("2. Green");
        println!("3. Yellow");
        println!("4. Red");
        let mut level = String::new();
        match std::io::stdin().read_line(&mut level){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_ambulance();},
        }
        let mut level:i32=match level.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_ambulance();
            },
        };
        match level{
            1=>level = 1,
            2=>level = 2,
            3=>level = 3,
            4=>level = 4,
            _=>println!("Invalid choice!")
        }
        
        self.vehicles.push(Box::new(Ambulance{
            id: id.trim().to_string(),
            position: (x,y),
            level: level,
            availability: true
        }));
    }
    /// This function adds a new helicopter to a list of vehicles, prompting the user for its ID,
    /// position, and level.
    fn add_helicopter(&mut self){
        println!("Insert the id of the helicopter: ");
        let mut id = String::new();
        match std::io::stdin().read_line(&mut id){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_helicopter();},
        }
        for vehicle in self.vehicles.iter(){
            if vehicle.get_id() == id.trim(){
                println!("A helicopter with this id already exists!, try again:");
                return self.add_helicopter()
            }
        }
    
        println!("Insert the x position of the helicopter: ");
        let mut x = String::new();
        match std::io::stdin().read_line(&mut x){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_helicopter();},
        }
        let x = match x.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_helicopter();
            },
        };
        println!("Insert the y position of the helicopter: ");
        let mut y = String::new();
        match std::io::stdin().read_line(&mut y){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_helicopter();},
        }
        let y = match y.trim().parse::<f64>(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_helicopter();
            },
        };
        println!("Insert the level of the helicopter: ");
        println!("1. White");
        println!("2. Green");
        println!("3. Yellow");
        println!("4. Red");
        let mut level = String::new();
        match std::io::stdin().read_line(&mut level){
            Ok(_)=>{},
            Err(_)=>{println!("Failed to read line!"); return self.add_helicopter();},
        }
        let mut level: i32 = match level.trim().parse(){
            Ok(num)=>num,
            Err(_)=>{println!("Please type a number!"); return self.add_helicopter();
            },
        };
        match level{
            1=>level = 1,
            2=>level = 2,
            3=>level = 3,
            4=>level = 4,
            _=>println!("Invalid choice!")
        }
        self.vehicles.push(Box::new(Helicopter{
            id: id.trim().to_string(),
            position: (x,y),
            level: level,
            availability: true
        }));
    
    }
    fn show_vehicles(&mut self){
        println!("Vehicles: ");
        for vehicle in self.vehicles.iter(){
            vehicle.display_details();
        }
    }
}

pub trait EmergencyVehicle{
    fn display_details(&self);
    fn get_id(&self) -> String;
    fn get_position(&self) -> (f64,f64);
    fn get_level(&self) -> i32;
    fn get_availability(&self) -> bool;
    fn set_availability(&mut self, availability: bool);
    fn get_type(&self) -> String;
    fn move_to(&mut self, position: (f64,f64));
    fn get_time(&self, position: (f64,f64))->f64;
}

pub struct Ambulance{
    id: String,
    position: (f64,f64),
    level: i32,
    availability: bool,
}

pub struct Helicopter{
    id: String,
    position: (f64,f64),
    level: i32,
    availability: bool,
}


impl EmergencyVehicle for Ambulance{
    fn display_details(&self){
        println!("Ambulance: {}, ({},{}), {}, {}", self.id, self.position.0, self.position.1, self.level, self.availability);
    }
    fn get_id(&self) -> String{
        self.id.clone()
    }
    fn get_position(&self) -> (f64,f64){
        self.position
    }
    fn get_level(&self) -> i32 {
        self.level
    }

    fn get_availability(&self) -> bool{
        self.availability
    }
    fn set_availability(&mut self, availability: bool){
        self.availability = availability;
    }
    fn get_type(&self) -> String{
        String::from("Ambulance")
    }


    fn get_time(&self, position: (f64,f64))->f64 {
            let speed: f64= 100.0;
            let distance= (position.0-self.position.0).abs() + (position.1-self.position.1).abs();
            let counter= (distance/speed) as f64;
            return counter;
        }
    fn move_to(&mut self, position: (f64,f64)){
            
            self.position = position;
        }
}
impl EmergencyVehicle for Helicopter{
    fn display_details(&self){
        println!("Helicopter: {}, ({},{}), {}, {}", self.id, self.position.0, self.position.1, self.level, self.availability);
    }
    fn get_id(&self) -> String{
        self.id.clone()
    }
    fn get_position(&self) -> (f64,f64){
        self.position
    }
    fn get_level(&self) -> i32 {
        self.level
        
    }

    fn get_time(&self, position: (f64,f64)) -> f64{
        let speed= 250.0;
       
        let distance= ((position.0 - self.position.0).powf(2.0) + (position.1 - self.position.1).powf(2.0)).sqrt();
        let counter= distance/speed;
        return counter + (5.0/60.0);
        
        
    }
    fn get_availability(&self) -> bool{
        self.availability
    }
    fn set_availability(&mut self, availability: bool){
        self.availability = availability;
    }
    fn get_type(&self) -> String{
        String::from("Helicopter")
    }
    fn move_to(&mut self, position: (f64,f64)){
        
        self.position = position;
    }
}