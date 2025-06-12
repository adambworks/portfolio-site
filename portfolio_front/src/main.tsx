import { createRoot } from "react-dom/client";
import "./index.css";
import { BrowserRouter, Routes, Route } from "react-router";
import ProjectsPage from "./pages/ProjectsPage.tsx";
import ProjectDetailPage from "./pages/ProjectDetailPage.tsx";
import ChapterPage from "./pages/ChapterPage.tsx";
import HomePage from "./pages/HomePage.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<HomePage />} />
      <Route path="/projects" element={<ProjectsPage />} />
      <Route path="/projects/:slug" element={<ProjectDetailPage />} />
      <Route path="/projects/:slug/chapter/:chapterIndex" element={<ChapterPage />}/>
    </Routes>
  </BrowserRouter>
);
