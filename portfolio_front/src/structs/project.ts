
export type Project = {
  id: number;
  name: string;
  date_started: string;
  overview: string | null;
  slug: string;
  image: string | null;
  chapter_descriptor: string | null; 
};

export type NewProject = {
  name: string;
  slug: string;
  date_started: string;
  overview: string | null;
  image: string | null;
  chapter_descriptor: string | null;
};