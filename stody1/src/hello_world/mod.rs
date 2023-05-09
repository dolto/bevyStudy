use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};

pub fn hello_world(){
    println!("Hello World!");
}

pub fn setup(mut commands:Commands){
    commands.spawn(
        ( Person{
        name: "Alex".to_string()
        },
        Employed{
            job: Job::Doctor,
        } ) 
        //이런식으로 괄호로 감싸서 두개의 컴포넌트를 가진 객체를 생성할 수 있음...?
        //정확하게 말하면 Alex라는 Person객체는 Job::Doctor기능을 한다고 할 수 있음

    );
    commands.spawn(Person{
        name: "Dolto".to_string()
    });
    commands.spawn(
        ( Person{
        name: "Bob".to_string()
        },
        Employed{
            job: Job::FireFighter,
        } ) 
    );
    commands.spawn(
        ( Person{
        name: "Charlie".to_string()
        },
        Employed{
            job: Job::Lawyer,
        } ) 
    );
    commands.spawn(
        ( Person{
        name: "David".to_string()
        },
        Employed{
            job: Job::Nours,
        } ) 
    );
    commands.spawn(
        ( Person{
        name: "Ellen".to_string()
        },
        Employed{
            job: Job::FireFighter,
        } ) 
    );
}

pub fn print_names(person_query: Query<&Person>){
    for person in person_query.iter(){
        println!("NAME: {}", person.name);
    }
}

pub fn people_with_jobs(
    person_query: Query<&Person, With<Employed>>
){
    for person in person_query.iter(){
        println!("{} as a job.", person.name);
    }
}
//이러면 Person객체중 Employed를 가지고 있는 객체만 필터한 것과 같음

pub fn people_ready_for_hire(
    person_query: Query<&Person, Without<Employed>>
){
    for person in person_query.iter(){
        println!("{} is ready for hire.", person.name);
    }
}

pub fn person_does_job(
    person_query: Query<(&Person, &Employed)>
){
    for (person, employed) in person_query.iter(){
        let job_name: &str = match employed.job{
            Job::Doctor => "의사", //한국어 테스트
            Job::FireFighter => "Fire Fighter",
            Job::Nours => "Nours",
            Job::Lawyer => "Lawyer"
        };
        println!("{0} is a {1}.", person.name, job_name);
    }
}

#[derive(Component)]
pub struct Person {
    pub name: String
}
//bevy에게 Person구조체는 Component라고 알림
//이를 통해 Query<&Person>으로 모든 Person을 가져올 수 있음


#[derive(Component)]
pub struct Employed{
    pub job: Job
}

#[derive(Debug)]
pub enum Job{
    Doctor,
    FireFighter,
    Lawyer,
    Nours
}


//이제 이런 기능들을 모아서 한번에 수행하는 플러그인을 생성
//기능들을 모아두고 이를 추상화 한 것
pub struct HelloWorldPlugin;
impl Plugin for HelloWorldPlugin {
    fn build(&self, app: &mut App){
        app.add_startup_system(setup)
        .add_system(print_names)
        .add_system(people_with_jobs)
        .add_system(people_ready_for_hire)
        .add_system(person_does_job);
    }
}