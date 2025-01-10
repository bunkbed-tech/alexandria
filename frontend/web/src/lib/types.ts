import { Data } from "dataclass"

export class Resource extends Data {
  id?: number
  title!: string
  description?: string
  year_published?: number
  thumbnail?: string
  api_id!: number
}

export class Tag extends Data {
  id!: number
  name!: string
}

export class Tagging extends Data {
  id!: number
  tag_id!: number
  resource_id!: number
}

export class BggThingsSearch extends Data {
  resources!: Resource[]
  errors!: string[]
}
