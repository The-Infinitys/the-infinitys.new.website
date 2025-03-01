import Image from "next/image";
import type { Metadata } from "next";
import "./page.css";

// Import Images
import infinite_icon from "../public/infinite-icon.svg";
import infinite_logo_white from "../public/infinite-logo-white.svg";
import infinite_logo_black from "../public/infinite-logo-black.svg";
import the_infinitys_logo from "../public/home/The-Infinitys.png"

export const metadata: Metadata = {
  title: "The Infinity's",
  description: "Created by Next.js",
};

function contents() {
  return (
    <>
      <section className="contents" style={{ display: "flex", justifyContent: "center", alignItems: "center" }}>
        <Image
          style={{
            width: "20vw",
            height: "20vw",
            aspectRatio: "1/1",
            borderRadius: "10vw",
            marginRight: "5vw",
            marginLeft: "5vw",
          }}
          src={the_infinitys_logo} alt={""} />
        <div style={{ width: "50vw" }}>
          <h1>こんにちは! The Infinity's です!</h1>
          <p>趣味がプログラミングと楽器演奏の無限です！</p>
          <p>主にプログラミングについて投稿します！</p>
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
