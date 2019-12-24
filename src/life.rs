mod conway{
    use std::collections::HashSet;
    fn get_bit_xy(str:i32, x:i32, y:i32) -> bool {
        if x<0||y<0||x>=5||y>=5
        {
            return false;
        }
        str&(1<<(y*5+x))!=0
    }
    fn set_bit_xy(str:i32,x:i32,y:i32) -> i32 {
        str|(1<<(y*5+x))
    }
    fn rem_bit_xy(str:i32,x:i32,y:i32) -> i32 {
        str^(1<<(y*5+x))
    }
    fn neighbours_xy(str:i32, x:i32,y:i32) -> i32 {
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

    pub fn simulate_life() ->i32
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
            if world_set.contains(&world)
            {
                break;
            }
            else {
                world_set.insert(world);
            }
        }
        return world;
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn part_one_test()  {
            assert_eq!(simulate_life(),18350099);
        }
        #[test]
        fn rem_test()
        {
            let mut str:i32= 0b00000001110000001010101110100011;
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
}
mod eris{
    #[derive(Clone)]
    struct Eris{
        world_stack: Vec<i32>
    }
    impl Eris{
        fn get_bit_xy(&self,x:i32, y:i32,z:i32) -> bool {
            if x<0||y<0||x>=5||y>=5||z<0||z>=400
            {
                return false;
            }
            self.world_stack[z as usize]&(1<<(y*5+x))!=0
        }
        fn set_bit_xy(&mut self, x:i32, y:i32, z:i32) {
            self.world_stack[z as usize]|=(1<<(y*5+x));
        }
        fn rem_bit_xy(&mut self, x:i32, y:i32, z:i32)  {
            self.world_stack[z as usize]^=(1<<(y*5+x));
        }
        fn neighbours_xy(&self,x:i32,y:i32,z:i32) -> i32 {
            let mut count = 0;
            if x==0{
                if self.get_bit_xy(1,2,z-1)
                {
                    count+=1;
                }
            }
            if y==0{
                if self.get_bit_xy(2,1,z-1)
                {
                    count+=1;
                }
            }
            if x==4{
                if self.get_bit_xy(3,2,z-1)
                {
                    count+=1;
                }
            }
            if y==4{
                if self.get_bit_xy(2,3,z-1)
                {
                    count+=1;
                }
            }


            if x==1&&y==2
            {
                for i in 0..5{
                    if self.get_bit_xy(0,i,z+1)
                    {
                        count+=1;
                    }
                }
            }
            if x==2&&y==1
            {
                for i in 0..5{
                    if self.get_bit_xy(i,0,z+1)
                    {
                        count+=1;
                    }
                }
            }
            if x==3&&y==2
            {
                for i in 0..5{
                    if self.get_bit_xy(4,i,z+1)
                    {
                        count+=1;
                    }
                }
            }
            if x==2&&y==3
            {
                for i in 0..5{
                    if self.get_bit_xy(i,4,z+1)
                    {
                        count+=1;
                    }
                }
            }
            if self.get_bit_xy(x-1,y,z)
            {
                count+=1;
            }
            if self.get_bit_xy(x+1,y,z)
            {
                count+=1;
            }
            if self.get_bit_xy(x,y-1,z)
            {
                count+=1;
            }
            if self.get_bit_xy(x,y+1,z)
            {
                count+=1;
            }
            count
        }
    }

    pub fn simulate_life() ->i32{
        let mut eris: Eris = Eris{world_stack: vec![0;400]};
        eris.world_stack[200]=0b1110000001010101110100011;
        for i in 0..200{
            let mut new_eris = eris.clone();
            for z in 0..400{
                for x in 0..5{
                    for y in 0..5{
                        if !(x==2&&y==2)
                        {
                            let neigh_count=eris.neighbours_xy(x,y,z);
                            if eris.get_bit_xy(x,y,z)
                            {
                                if neigh_count!=1
                                {
                                    new_eris.rem_bit_xy(x,y,z);
                                }
                            }
                            else {
                                if neigh_count==1||neigh_count==2
                                {
                                    new_eris.set_bit_xy(x,y,z);
                                }
                            }
                        }
                    }
                }
            }
            eris=new_eris;
        }
        let mut count=0;
        for z in 0..400{
            for x in 0..5{
                for y in 0..5{
                    if eris.get_bit_xy(x,y,z)
                    {
                        count+=1;
                    }
                }
            }
        }
        count
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn part_two_test()  {
            assert_eq!(simulate_life(),2037);
        }
        #[test]
        fn neigh_test(){
            let mut eris: Eris = Eris{world_stack: vec![0;200]};
            eris.world_stack[1]=0b1111111111110111111111111;
            assert_eq!(eris.neighbours_xy(0,0,2),2);
            assert_eq!(eris.neighbours_xy(0,1,2),1);
            assert_eq!(eris.neighbours_xy(1,0,2),1);
            assert_eq!(eris.neighbours_xy(1,2,1),3);
            assert_eq!(eris.neighbours_xy(2,3,1),3);
        }
        #[test]
        fn inner_test(){
            let mut eris: Eris = Eris{world_stack: vec![0;200]};
            eris.world_stack[1]=0b1111111111110111111111111;
            assert_eq!(eris.neighbours_xy(1,2,1),3);
            assert_eq!(eris.neighbours_xy(2,3,1),3);
        }
    }
}