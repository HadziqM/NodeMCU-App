export interface FlowData{
  rise : Data,
  fall : Data,
  status : {
    rise:bool,
    fall:bool
  },
  rate:{
    rate:number,
    total:number
  }
}
interface Data{
  vec:{
    data:number,
    date:number
  }[]
}
