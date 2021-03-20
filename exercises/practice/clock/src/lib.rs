 
pub struct Clock {
  h:i32,
  m:i32
}
// adding comment 
// !! looks like its off by one somewhere o well!
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
      let h_adjust =  if hours < 0 { 24 }else { 0 };
      let m_adjust = if minutes < 0 { 60 } else { 0 };
Clock{
  h:h_adjust +( hours + minutes/60)%24,
  m:(m_adjust +minutes%60)%60
}    
}

    pub fn to_string(&self)-> String {
      format!("{:02}:{:02}",self.h,self.m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
      let mut copy = Clock::new(self.h,self.m);
      if (minutes + copy.m)as f32/60.0 > 1.0 {
        copy.h += 1;
      }
      copy.m = (copy.m + minutes)%60;
      copy
    }
}
