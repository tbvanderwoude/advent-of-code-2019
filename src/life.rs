use std::collections::HashSet;

fn get_bit_xy(str:i32, x:i32, y:i32) -> bool
{
    if x<0||y<0||x>=5||y>=5
    {
        return false;
    }
    str&(1<<(y*5+x))!=0
}
fn set_bit_xy(str:i32,x:i32,y:i32) -> i32
{
    str|(1<<(y*5+x))
}
fn rem_bit_xy(str:i32,x:i32,y:i32) -> i32
{
    str^(1<<(y*5+x))
}
fn neighbours_xy(str:i32, x:i32,y:i32) -> i32
{
    let mut count = 0;
    if get_bit_xy(str,x-1,y)
    {
        count+=1;
    }
    if get_bit_xy(str,x+1,y)
    {
        count+=1;
    }
    if get_bit_xy(str,x,y-1)
    {
        count+=1;
    }
    if get_bit_xy(str,x,y+1)
    {
        count+=1;
    }
    count
}
pub fn simulate_life()
{
    let mut world_set:HashSet<i32> = HashSet::new();
    let mut world:i32= 0b00000001110000001010101110100011;

    let mut new_world:i32 = 0;
    loop {
        new_world=world;
        for x in 0..5{
            for y in 0..5{
                let neigh_count=neighbours_xy(world,x,y);
                if get_bit_xy(world,x,y)
                {
                    if neigh_count!=1
                    {
                        new_world=rem_bit_xy(new_world,x,y);
                    }
                }
                else {
                    if neigh_count==1||neigh_count==2
                    {
                        new_world=set_bit_xy(new_world,x,y);
                    }
                }
            }
        }
        world=new_world;
        for y in 0..5{
            for x in 0..5{
                if get_bit_xy(world,x,y)
                {
                    print!("#");
                }
                else {
                    print!(".");
                }
            }
            println!();
        }
        if world_set.contains(&world)
        {
            println!("{}",world);
            break;
        }
        else {
            world_set.insert(world);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test()  {
        simulate_life();
    }
    #[test]
    fn rem_test()
    {
        let mut str:i32= 0b00000001110000001010101110100011;
        //assert_eq!(0b00000001110000001010101110100010,rem_bit_xy(str,0,0,));
        assert_eq!(0b00000001110000001010101110000011,rem_bit_xy(str,0,1,));
        assert_eq!(0b00000001110000001010101100100011,rem_bit_xy(str,2,1,));
    }
    #[test]
    fn add_test()
    {
        assert_eq!(0b00000001110000001010101110100011,set_bit_xy(0b00000001110000001010101110000011,0,1,));
        assert_eq!(0b00000001110000001010101110100011,rem_bit_xy(0b00000001110000001010101100100011,2,1,));
    }
}
