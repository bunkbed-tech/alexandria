export class Resource {
  public id?: number
  public title!: string
  public description?: string
  public year_published?: number
  public thumbnail?: string
  public bgg_id!: number
}

export class Tag {
  public id!: number
  public name!: string
}

export class Tagging {
  public id!: number
  public tag_id!: number
  public resource_id!: number
}
