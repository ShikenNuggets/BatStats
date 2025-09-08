"use client"

import TimeTable from "@/components/TimeTable";
//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";

type DataPair = [string, number];

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/236a84140c883ee6aa4f28bc3d8ae973/raw/42a3e074c2933cb10b68dfca49902b1944fa2c3c/BatStats_AnyTimes.json";

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
      <h1 style={{ paddingTop: '10px', textAlign: 'center' }}>Batman Arkham Speedrunning Stats</h1>
      <TimeTable data={data} title="Overall Any% Times" tableKey="Runner" tableValue="Time" />
    </div>
  );
}
