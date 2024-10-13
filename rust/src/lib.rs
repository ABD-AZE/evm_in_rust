use std::any::type_name;

use primitive_types::U256;
use primitive_types::U512;
pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn neg(f:& mut U256)-> bool{  // converts f to 2's complement if it is a negative number and returns true for a negative number
    if *f&(U256::from(1)<<255)!=U256::from(0){
        twoscomplement(f);
        return true
    }
    false
}
fn twoscomplement(f:& mut U256){
    let mut x=U256::from(1)<<255;
    x+=((U256::from(1)<<255)-1);
    *f-=U256::from(1);
    *f=*f^x;
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut stack: Vec<U256> = Vec::new();
    let mut pc = 0;

    let code = _code.as_ref();

    while pc < code.len() {
        let opcode = code[pc];
        pc += 1;

        if opcode == 0x00 {
            break;
            // STOP
        }
        else if opcode>=0x5f && opcode<=0x7F{
            let mut element  = opcode-0x5f;
            let mut f = U256::from(0);

            while element>0{
                let x=f.checked_mul(U256::from(256));
                match x{
                    Some(y)=> f=y,
                    None => f=U256::from(0),
                }
                f+=U256::from(code[pc]);
                pc+=1;
                if element!=0{

                    element-=1;
                }

            }
            stack.push(f);
        }
        else if opcode==0x50 {
            stack.pop();
        }
        else if opcode  == 0x01{
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=f.overflowing_add(g).0;
            stack.push(f);
        }
        else if opcode==0x02{
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=f.overflowing_mul(g).0;
            stack.push(f);
        }
        else if opcode == 0x03{
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=f.overflowing_sub(g).0;
            stack.push(f);
        }
        else if opcode==0x04{
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=match f.checked_div(g){
                Some(y)=>y,
                None=> U256::from(0),
            };
            stack.push(f);
        }
        else if opcode==0x06{
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=match f.checked_rem(g){
                Some(y)=>y,
                None=>U256::from(0),
            };
            stack.push(f);
        }
        else if opcode==0x07{
            let mut f=stack.pop().unwrap();
            let mut g=stack.pop().unwrap();
            let mut t=false;
            if(neg(&mut f)){
                t=true;
            }
            if(neg(&mut g)){
                t=true;
            }
            if g==U256::from(0){
                stack.push(g);
            }
            else{
                f=match f.checked_rem(g){
                    Some(y)=>y,
                    None=>U256::from(0),
                };
                
                if(t){
                    twoscomplement(&mut f);
                }
                stack.push(f);
            }
            
        }
        else if opcode==0x08{
            let mut f=stack.pop().unwrap();
            let mut g=stack.pop().unwrap();
            let m=stack.pop().unwrap();
            f=match f.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            g=match g.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            f=f.overflowing_add(g).0;
            f=match f.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            stack.push(f);
        }
        else if opcode==0x09{
            let mut f=stack.pop().unwrap();
            let mut g: U256=stack.pop().unwrap();
            let m=stack.pop().unwrap();
            f=match f.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            g=match g.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            f=f.overflowing_mul(g).0;
            f=match f.checked_rem(m){
                Some(y)=>y,
                None=>U256::from(0),
            };
            stack.push(f);
        }
        else if opcode==0x0a {
            let mut f=stack.pop().unwrap();
            let g=stack.pop().unwrap();
            f=f.overflowing_pow(g).0;
            stack.push(f);    
        }
        else if opcode==0x0b {
            let g = stack.pop().unwrap();
            let mut f: U256 = stack.pop().unwrap();
            if(g!=U256::from(0)){
                stack.push(f);
            }
            else{
                if(f&(U256::from(1)<<7)==U256::from(0)){
                    stack.push(f);
                }
                else{
                    let mut x=U256::from(1)<<255;
                    x+=((U256::from(1)<<255)-1);
                    let y = f&x;
                    x-=y;
                    f+=x;
                    stack.push(f);
                }
            }
        }
        else if opcode==0x05 {
            let mut f=stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            if(g==U256::from(0)){
                stack.push(g);
            }
            else{
                let mut x=U256::from(1)<<255;
                x+=((U256::from(1)<<255)-1);
                let mut t=false;
                if(f&(U256::from(1)<<255)!=U256::from(0)){
                    t=true;
                    f-=U256::from(1);
                    f=f^x;
                }
                if(g&(U256::from(1)<<255)!=U256::from(0)){
                    t=!t;
                    g-=U256::from(1);
                    g=g^x;
                }
                f=f.checked_div(g).unwrap();
                if(t){
                    f=f^x;
                    f+=U256::from(1);
                }
                stack.push(f);
            }
        }
        else if opcode==0x10 {
            let mut f = stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            if f<g{
                stack.push(U256::from(1));
            }
            else {stack.push(U256::from(0));}
        }
        else if opcode==0x11{
            let mut f = stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            if g<f {
                stack.push(U256::from(1));
                
            }
            else {
                stack.push(U256::from(0));
            }
        }
        else if opcode == 0x12{
            let mut f = stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            let a=neg(& mut f);
            let b= neg(& mut g);
            if(a&&b){
                if f>g {
                    stack.push(U256::from(1));
                }
                else {
                        stack.push(U256::from(0));
                }
            }
            else if(a&&!b){
                stack.push(U256::from(1));
            }
            else if(!a&&b){
                stack.push(U256::from(0));
            }
            else{
                if f<g{
                    stack.push(U256::from(1));
                }
                else {stack.push(U256::from(0));}
            }
        }
        else if opcode==0x13 {
            let mut f = stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            let temp = f;
            f=g;
            g=temp;
            let a=neg(& mut f);
            let b= neg(& mut g);
            if(a&&b){
                if f>g {
                    stack.push(U256::from(1));
                }
                else {
                        stack.push(U256::from(0));
                }
            }
            else if(a&&!b){
                stack.push(U256::from(1));
            }
            else if(!a&&b){
                stack.push(U256::from(0));
            }
            else{
                if f<g{
                    stack.push(U256::from(1));
                }
                else {stack.push(U256::from(0));}
            }
        }
        else if opcode==0x14 {
            let mut f = stack.pop().unwrap();
            let mut g = stack.pop().unwrap();
            if(f==g){
                stack.push(U256::from(1));
            }
            else{
                stack.push(U256::from(0));
            }
        }
        else if opcode==0x15{
            let mut f = stack.pop().unwrap();
            if(f==U256::from(0)){
                stack.push(U256::from(1));
            }
            else{
                stack.push(U256::from(0));
            }
        }
        else if opcode==0x19{
            let mut f = stack.pop().unwrap();
            f+=U256::from(1);
            twoscomplement(&mut f);
            stack.push(f);
        }
        else if opcode==0x16{
            let mut f = stack.pop().unwrap();
            let mut g= stack.pop().unwrap();
            f=f&g;
            stack.push(f);
        }
        else if opcode==0x17{
            let mut f = stack.pop().unwrap();
            let mut g= stack.pop().unwrap();
            f=f|g;
            stack.push(f);
        }
        else if opcode==0x18{
            let mut f = stack.pop().unwrap();
            let mut g= stack.pop().unwrap();
            f=f^g;
            stack.push(f);
        }
        else if opcode==0x1b{

        }
    }
    
    // TODO: Implement me
    stack.reverse();
    return EvmResult {
        stack: stack,
        success: true,
    };
}
