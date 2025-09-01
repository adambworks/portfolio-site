export interface Chapter {
    id: number;
    project_id: number;
    name: string;
    date_started: string | null;
    index: number;
  }
  
  export interface NewChapter {
    project_id: number;
    name: string;
    date_started: string | null;
    index: number;
  }