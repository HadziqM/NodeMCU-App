export interface FlowData{
  rise : {
    value:number,
    created_at:number
  }[],
  fall : {
    value:number,
    created_at:number
  }[],
  status : {
    rise:bool,
    fall:bool
  }
}
