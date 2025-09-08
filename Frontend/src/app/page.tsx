"use client"

//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";

type DataPair = [string, number];

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/236a84140c883ee6aa4f28bc3d8ae973/raw/42a3e074c2933cb10b68dfca49902b1944fa2c3c/BatStats_AnyTimes.json";

function formatSeconds(seconds: number): string{
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${pad(hrs)}:${pad(mins)}:${pad(secs)}`;
}

export default function Home() {
  const [data, setData] = useState<DataPair[] | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setLoading] = useState(true);

  useEffect(() => {
    const fetchData = async() => {
      try{
        const result = await fetch(GIST_RAW_URL);
        if (!result.ok){
          throw new Error("Failed to fetch: ${res.status}");
        }

        const json = await result.json();
        setData(json as DataPair[]);
      }catch(err: any){
        setError(err);
      }finally{
        setLoading(false);
      }
    };

    fetchData();
  }, []);

  if(isLoading){
    return <div>Loading...</div>;
  }else if(error){
    return <div style={{color: 'red' }}>Error: {error} - please try again later</div>;
  }else if(!data){
    return <div>No data available - please try again later.</div>;
  }

  return (
    <div>
      <h1>Overall Any% Times</h1>
      <table border={1} cellPadding={8}>
        <thead>
          <tr>
            <th>Runner</th>
            <th>Time</th>
          </tr>
        </thead>
        <tbody>
          {
            data.map(([name, value], index) => (
              <tr key={index}>
                <td>{name}</td>
                <td>{formatSeconds(value)}</td>
              </tr>
            ))
          }
        </tbody>
      </table>
    </div>
  );
}
