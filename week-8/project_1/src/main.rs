use std::io;

fn main() {
    
    let a = "APS 1-2";
    let b = "APS 3-5";
    let c = "APS 5-8";
    let d = "EL1 8-10";
    let e = "EL2 10-13";
    let f = "SES";
    let teacher = vec!["Placement", "Classroom teacher", "Senior teacher", "Leading teacher", "Deputy principal", "Principal"];
    let val1 = teacher[0];
    let val2 = teacher[1];
    let val3 = teacher[2];
    let val4 = teacher[3];
    let val5 = teacher[4];
    let val6 = teacher[5];
    let lawyer = vec!["Paralegal", "Junior associate", "Associate", "Senior associate 1-2", "Senior associate 3-4", "Partner"];
    let val7 = lawyer[0];
    let val8 = lawyer[1];
    let val9 = lawyer[2];
    let val10 = lawyer[3];
    let val11 = lawyer[4];
    let val12 = lawyer[5];
    let academic = vec!["Research asst", "PhD Candiditae", "Postdoc Researcher", "Senir Lecturer", "Dean"];
    let val13 = academic[0];
    let val14= academic[1];
    let val15= academic[2];
    let val16 = academic[3];
    let val17 = academic[4];
    let office_admin = vec!["Intern", "Administrator", "Senior Administrator", "Office Manager", "Director", "CEO"];
    let val18 = office_admin[0];
    let val19 = office_admin[1];
    let val20 = office_admin[2];
    let val21 = office_admin[3];
    let val22 = office_admin[4];
    let val23 = office_admin[5];


    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Are you a:
        1. teacher.
        2. Lawyer.
        3. Academic.
        4. Office administrator.
        signify using the number options");
    io::stdin().read_line(&mut input1).expect("FAILED");

    if input1.trim() == "1"   {

    println!("How many years of experience do you have?
        1. 1-2 years.
        2. 3-5 years.
        3. 5-8 years.
        4. 8-10 years.
        5. 10-13 years.
        6. SES
        Signify using the number options.");
    io::stdin().read_line(&mut input2).expect("FAILED");
    let exp:u64 = input2.trim().parse().expect("FAILED");

    if input2.trim() == "1"   {
        println!("You are a {} teacher on {} level", val1, a);
    }

    else if input2.trim() == "2"  {
        println!("You are a {} teacher on {} level", val2, b);
    }

    else if input2.trim() == "3"  {
        println!("You are a {} teacher on {} level", val3, c);
    }

    else if input2.trim() == "4"    {
        println!("You are a {} teacher on a {} level", val4, d);
    }

    else if input2.trim() == "5"    {
        println!("You are a {} teacher on a {} level", val5, e);
    }

    else if input2.trim() == "6"   {
        println!("You are a {} teacher on a {} level", val6, f)
    }
}

    else if input1.trim() == "2"  {
         println!("How many years of experience do you have?
        1. 1-2 years.
        2. 3-5 years.
        3. 5-8 years.
        4. 8-10 years.
        5. 10-13 years.
        6. SES
        Signify using the number options.");
         io::stdin().read_line(&mut input2).expect("FAILED");

       if input2.trim() == "1"   {
        println!("You are a {} lawyer on {} level", val7, a);
    }
    else if input2.trim() == "2"   {
        println!("You are a {} lawyer on {} level", val8, b);
    }
    else if input2.trim() == "3"   {
        println!("You are a {} lawyer on {} level", val9, c);
    }
    else if input2.trim() == "4"   {
        println!("You are a {} lawyer on {} level", val10, d);
    }
    else if input2.trim() == "5"   {
        println!("You are a {} lawyer on {} level", val11, e);
    }
    else if input2.trim() == "6"   {
        println!("You are a {} lawyer on {} level", val12, f);
    }

    }
    

     else if input1.trim() == "3"  {
         println!("How many years of experience do you have?
        1. 1-2 years.
        2. 3-5 years.
        3. 5-8 years.
        4. 8-10 years.
        5. 10-13 years.
        6. SES
        Signify using the number options.");
         io::stdin().read_line(&mut input2).expect("FAILED");

       if input2.trim() == "1"   {
        println!("You have no available qualifications");
    }
    else if input2.trim() == "2"   {
        println!("You are a {}  on {} level", val13, b);
    }
    else if input2.trim() == "3"   {
        println!("You are a {}  on {} level", val14, c);
    }
    else if input2.trim() == "4"   {
        println!("You are a {}  on {} level", val15, d);
    }
    else if input2.trim() == "5"   {
        println!("You are a {}  on {} level", val16, e);
    }
    else if input2.trim() == "6"   {
        println!("You are a {}  on {} level", val17, f);
    }

    }


    else if input1.trim() == "4"  {
         println!("How many years of experience do you have?
        1. 1-2 years.
        2. 3-5 years.
        3. 5-8 years.
        4. 8-10 years.
        5. 10-13 years.
        6. SES
        Signify using the number options.");
         io::stdin().read_line(&mut input2).expect("FAILED");

       if input2.trim() == "1"   {
        println!("You are a {}  on {} level", val18, a);
    }
    else if input2.trim() == "2"   {
        println!("You are a {}  on {} level", val19, b);
    }
    else if input2.trim() == "3"   {
        println!("You are a {}  on {} level", val20, c);
    }
    else if input2.trim() == "4"   {
        println!("You are a {}  on {} level", val21, d);
    }
    else if input2.trim() == "5"   {
        println!("You are a {}  on {} level", val22, e);
    }
    else if input2.trim() == "6"   {
        println!("You are a {}  on {} level", val23, f);
    }

    }




    
    


}
