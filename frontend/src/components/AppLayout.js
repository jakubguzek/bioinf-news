import { Outlet } from "react-router-dom";
import Footer from "./Footer";

export default function AppLayout() {
    return (
        <div className="app-layout">
            <div className="app-page">
                <Outlet/>
            </div>
            <Footer/>
        </div>
    );
  }