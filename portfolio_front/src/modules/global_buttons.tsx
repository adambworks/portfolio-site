/* eslint-disable @typescript-eslint/no-unused-vars */
import { useState } from "react";
import type { Project } from "../structs/project";
import type { Chapter } from "../structs/chapter";
import { Link } from "react-router";

export default function HamburgerMenu(projects: Project[] ) {
//   const location = useLocation();
  const [open, setOpen] = useState(false);

  return (
    <div className="text-colorc px-4 py-3">
      <div className="flex justify-between items-center">
     
        <button
          className=" text-colorc focus:outline-none"
          onClick={() => setOpen(!open)}
        >
          ☰
        </button>


      </div>

      {open && (
        <div className="mt-2 flex flex-col gap-2">
            <Link
           //   key=""
    to="/" className="hover:underline" onClick={() => setOpen(false)}>
              Home
            </Link>
            <Link to="/projects" className="hober:underline" onClick={() => setOpen(false)}>
            Projects
            </Link>
            {projects.map((project) =>(
                <span className="ml-64 text-left">
                <Link to={`/projects/${project.slug}`} className="hover:underline" onClick={() => setOpen(false)}>
            - {project.name}
            </Link>
            </span>
            ))}
        </div>
      )}
    </div>
  );
}
