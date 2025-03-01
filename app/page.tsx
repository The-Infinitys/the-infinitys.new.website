import Image from "next/image";
import type { Metadata } from "next";
import "./page.css";

// Import Images
import infinite_icon from "../public/infinite-icon.svg";
import infinite_logo_white from "../public/infinite-logo-white.svg";
import infinite_logo_black from "../public/infinite-logo-black.svg";

export const metadata: Metadata = {
  title: "The Infinity's",
  description: "Created by Next.js",
};

function contents() {
  return (
    <>
      <section className="second">
        <div>
          <Image src={infinite_icon} alt={""} />
        </div>
      </section>
    </>);
}

export default function Home() {
  return (
    <>
      <div className="first-view">
        <Image className="inf-icon" src={infinite_icon} alt="" />
        <Image className="dark-mode inf-logo" src={infinite_logo_white} alt="" />
        <Image className="light-mode inf-logo" src={infinite_logo_black} alt="" />
      </div>
      {contents()}
    </>
  );
}
