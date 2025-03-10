import fs from "fs";
import path from "path";
import React from "react";
import Image from "next/image";
import Link from "next/link";
import "./page.css";
import loading_image from "../../public/article/loading.svg";

type Article = {
  date: string;
  id: string;
  thumbnail: string;
  title: string;
};

type Month = [] | Article[];

type Year = {
  [key: string]: Month;
};

type ArticleData = {
  [key: string]: Year;
};

const getArticleData = async (): Promise<ArticleData> => {
  const filePath = path.join(
    process.cwd(),
    "public/article/database/article-data.json"
  );
  const jsonData = fs.readFileSync(filePath, "utf8");
  const articles: ArticleData = JSON.parse(jsonData);
  return articles;
};

const getImageData = (thumb_path: string) => {
  try {
    return require(thumb_path);
  } catch (error) {
    console.error("Error loading image:", error);
    return null;
  }
};

export default async function Page() {
  const articleData: ArticleData = await getArticleData();
  const years = Object.keys(articleData).sort(
    (a, b) => parseInt(b) - parseInt(a)
  );
  return (
    <>
      <section className="first">
        {years.map((year) => {
          const months = Object.keys(articleData[year]).sort((a, b) => {
            const monthOrder = [
              "January",
              "February",
              "March",
              "April",
              "May",
              "June",
              "July",
              "August",
              "September",
              "October",
              "November",
              "December",
            ];
            return monthOrder.indexOf(a) - monthOrder.indexOf(b);
          });
          return (
            <div key={year}>
              <h2>{year}</h2>
              {months.map((month) => {
                return (
                  <div key={month}>
                    <h3>{month}</h3>
                    {articleData[year][month] &&
                      articleData[year][month].map((article: Article) => {
                        const article_path: string = `/article-${year}/${month}/${article.id}`;
                        const thumb_path: string = `public/article/database/article-${year}/${month}/${article.id}/${article.thumbnail}`;
                        const image_data = getImageData(thumb_path);
                        return (
                          <div key={article.id}>
                            <Link href={article_path}>
                              {image_data && (
                                <Image
                                  src={loading_image}
                                  alt={article.title}
                                  width={400}
                                  height={300}
                                />
                              )}
                              <h4>{article.title}</h4>
                              <p>{article.date}</p>
                            </Link>
                          </div>
                        );
                      })}
                  </div>
                );
              })}
            </div>
          );
        })}
      </section>
    </>
  );
}
