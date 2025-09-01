export interface Entry {
    id: number;
    chapter_id: number;
    text: string| null;
    date: string| null;
    image: string| null;
    index: number;
  }

  export interface NewEntry {
    chapter_id: number;
    text: string| null;
    date: string| null;
    image: string| null;
    index: number;
  }