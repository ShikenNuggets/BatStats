"use client"

import TimeTable, { ValueType } from "@/components/TimeTable";
//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";

type Metadata = {
  date: Date;
}

type DataPair = {
  rank: number;
  player: string;
  value: number;
};

interface BatStatsData {
  meta: Metadata;
  world_records: DataPair[];
  runner_times: DataPair[];
  runner_ranks: DataPair[];
  any_times: DataPair[];
  glitchless_times: DataPair[];
  hundo_times: DataPair[];
  asylum_mastery: DataPair[];
  city_mastery: DataPair[];
  origins_mastery: DataPair[];
  knight_mastery: DataPair[];
  overall_mastery: DataPair[];
}

type DataKey = keyof BatStatsData;

interface DatasetConfig {
  key: DataKey;
  buttonLabel: string;
  tableTitle: string;
  tableValue: string;
  valueType: ValueType;
  explanation: string | undefined;
}

const anyPercentExplanation = "Best full-game NG time (Any%, Glitchless, or 100%) in each game, added together.";
const glitchlessExplanation = "Best full-game Glitchless time in each game, added together.";
const hundoExplanation = "Best 100% time in each game, added together.";

const runnerTimesExplanation = "Overall Time represents how far you are from World Record across all categories. 0:00:00 would mean you have World Record in every category. Not having a run in a category is treated as equivalent to last place. Miscellaneous categories are excluded.";
const runnerTanksExplanation = "Overall Rank represents how far you are in ranks from World Record across all categories. 0 would mean you have World Record in every category. Not having a run in a category is treated as equivalent to last place. Miscellaneous categories are excluded.";

const GIST_RAW_URL = "https://gist.githubusercontent.com/ShikenNuggets/3adaa36be92dfb82f43b951b91387c1a/raw/BatStats.json";

export default function Home() {
  const [data, setData] = useState<BatStatsData | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setLoading] = useState(true);
  const [selectedKey, setSelectedKey] = useState<DataKey>('any_times');

  const datasetConfigs: DatasetConfig[] = [
    { key: 'any_times', buttonLabel: 'Any%', tableTitle: 'Overall Any% Times', tableValue: "Time", valueType: ValueType.Seconds, explanation: anyPercentExplanation },
    { key: 'glitchless_times', buttonLabel: 'Glitchless', tableTitle: 'Overall Glitchless Times', tableValue: "Time", valueType: ValueType.Seconds, explanation: glitchlessExplanation  },
    { key: 'hundo_times', buttonLabel: '100%', tableTitle: 'Overall 100% Times', tableValue: "Time", valueType: ValueType.Seconds, explanation: hundoExplanation  },
    { key: 'asylum_mastery', buttonLabel: 'Asylum', tableTitle: 'Asylum Mastery', tableValue: "Mastery", valueType: ValueType.Percent, explanation: undefined },
    { key: 'city_mastery', buttonLabel: 'City', tableTitle: 'City Mastery', tableValue: "Mastery", valueType: ValueType.Percent, explanation: undefined  },
    { key: 'origins_mastery', buttonLabel: 'Origins', tableTitle: 'Origins Mastery', tableValue: "Mastery", valueType: ValueType.Percent, explanation: undefined },
    { key: 'knight_mastery', buttonLabel: 'Knight', tableTitle: 'Knight Mastery', tableValue: "Mastery", valueType: ValueType.Percent, explanation: undefined },
    { key: 'overall_mastery', buttonLabel: 'Overall', tableTitle: 'Overall Mastery', tableValue: "Mastery", valueType: ValueType.Percent, explanation: undefined },
    { key: 'world_records', buttonLabel: 'WRs', tableTitle: '# of World Records', tableValue: "WRs", valueType: ValueType.Count, explanation: undefined  },
    { key: 'runner_times', buttonLabel: 'Times', tableTitle: 'Overall Leaderboard Times', tableValue: "Time", valueType: ValueType.Seconds, explanation: runnerTimesExplanation  },
    { key: 'runner_ranks', buttonLabel: 'Ranks', tableTitle: 'Overall Leaderboard Ranks', tableValue: "Rank", valueType: ValueType.Count, explanation: runnerTanksExplanation  },
  ];

  useEffect(() => {
    const fetchData = async() => {
      try{
        const result = await fetch(GIST_RAW_URL);
        if (!result.ok){
          throw new Error("Failed to fetch: ${res.status}");
        }

        const json = await result.json();
        setData(json as BatStatsData);
      }catch(err: unknown){
        if (err instanceof Error){
          setError(err.message);
        }else{
          setError("Unknown error");
        }
        
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

  const date = new Date(data.meta.date);

  return (
    <div>
      <h1 style={{ paddingTop: '10px', textAlign: 'center' }}>Batman Arkham Speedrunning Stats</h1>

      <h4 style={{ display: 'flex', flexWrap: 'wrap', justifyContent: 'center', gap: '10px', margin: '20px 0' }}>Last Updated: {date.toLocaleDateString('en-US', { year: 'numeric', month: 'long', day: 'numeric' })}</h4>

      {/* Buttons */}
      <div style={{ display: 'flex', flexWrap: 'wrap', justifyContent: 'center', gap: '10px', margin: '20px 0' }}>
        {datasetConfigs.map((btn) => (
          <button
            key={btn.key}
            onClick={() => setSelectedKey(btn.key)}
            style={{
              padding: '10px 15px',
              cursor: 'pointer',
              backgroundColor: selectedKey === btn.key ? '#444' : '#eee',
              color: selectedKey === btn.key ? 'white' : 'black',
              border: '1px solid #ccc',
              borderRadius: '5px',
              transition: 'background-color 0.2s',
            }}
          >
            {btn.buttonLabel}
          </button>
        ))}
      </div>

      <div style={{ display: 'flex', flexWrap: 'wrap', alignContent: 'center', justifyContent: 'center', gap: '10px', margin: '20px 0' }}>
      <TimeTable
        data={data[selectedKey] as DataPair[]}
        title={datasetConfigs.find((b) => b.key === selectedKey)?.tableTitle || 'Data'}
        tableKey="Runner"
        tableValue={datasetConfigs.find((b) => b.key === selectedKey)?.tableValue || 'Value'}
        valueType={datasetConfigs.find((b) => b.key === selectedKey)?.valueType}
        explanation={datasetConfigs.find((b) => b.key === selectedKey)?.explanation || ''}
      />
      </div>
    </div>
  );
}
