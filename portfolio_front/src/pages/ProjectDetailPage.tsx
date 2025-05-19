import { Link, useParams } from "react-router";
import { useEffect, useState } from "react";
import { fetchProjectBySlug } from "../api/projects";
import type { Project } from "../structs/project";
import type { Chapter } from "../structs/chapter";
import { fetchChapters } from "../api/chapters";


function chapter_descriptor(project: Project, chapter_index: number){
    if (project.chapter_descriptor !=null){
        return (<h2>{project.chapter_descriptor}: {chapter_index}</h2>);

    }
    else{
        return;
    }    
}


export default function ProjectDetailPage() {
  const { slug } = useParams<{ slug: string }>();
  const [project, setProject] = useState<Project | null>(null);
  const [chapters, setChapters] = useState<Chapter[]>([]);

  useEffect(() => {
    if (slug) {
      fetchProjectBySlug(slug).then(setProject).catch(console.error);
    }
  }, [slug]);

  useEffect(() => {
    if (!project) return;
    fetchChapters(project.id).then(setChapters).catch(console.error);
  }, [project]);

  if (!project) return <div className="p-4"> Invalid project</div>;



  const chapterList = chapters.length > 0 ? (
    chapters.map((chapter) => (
      <Link
        key={chapter.index}
        to={`/projects/${project.slug}/chapter/${chapter.index}`}
        className="block p-2 rounded-lg shadow hover:shadow-md transition bg-white border border-gray-200"
      >
        {chapter_descriptor(project,chapter.index)}
        <h2 className="text-lg font-semibold mb-1">{chapter.name}</h2>
      </Link>
    ))
  ) : (
    <p className="text-gray-500">No chapters available.</p>
  );

  return (
    <div className="p-4">
        debug project id: {project.id}
      {project.image !== null && project.image != "" && (
        <img
          src={`http://localhost:8080/api/images/${project.image}`}
          alt=""
          className="mb-2 w-full h-40 object-cover rounded"
        />
      )}
      <h1 className="text-2xl font-bold mb-2">{project.name}</h1>
      <p className="text-gray-500 mb-4">{project.date_started}</p>
      <p>overview: {project.overview}</p>
      <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
        {chapterList}
      </div>
    </div>
  );
}
