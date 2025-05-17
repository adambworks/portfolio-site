import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { BrowserRouter, Routes, Route } from "react-router";
import ProjectsPage from "./pages/ProjectsPage.tsx";
//import ProjectsPage from "./pages/ProjectsPage.tsx";

createRoot(document.getElementById("root")!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/test" element={<App />} />
      <Route path="/projects" element={<ProjectsPage />} />
    </Routes>
  </BrowserRouter>
);
