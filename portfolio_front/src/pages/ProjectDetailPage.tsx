import { useParams } from "react-router";
import { useEffect, useState } from "react";
import { fetchProjectBySlug } from "../api/projects";
import type { Project } from "../structs/project";

export default function ProjectDetailPage() {
  const { slug } = useParams<{ slug: string }>();
  const [project, setProject] = useState<Project | null>(null);

  useEffect(() => {
    if (slug) {
      fetchProjectBySlug(slug).then(setProject).catch(console.error);
    }
  }, [slug]);

  if (!project) return <div className="p-4"> Invalid project</div>;

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-2">{project.name}</h1>
      <p className="text-gray-500 mb-4">{project.date_started}</p>
      <p>{project.overview}</p>
    </div>
  );
}
