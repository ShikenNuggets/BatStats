"use client"

//import Image from "next/image";
//import styles from "./page.module.css";
import { useEffect, useState } from "react";
import axios from "axios";

export default function Home() {
  const [number, setNumber] = useState(null)

  useEffect(() => {
    axios.get("http://localhost:3001/random")
      .then(response => setNumber(response.data.value))
      .catch(err => console.error("Error fetching number: ", err));
  }, []);

  return (
    <div style={{ fontFamily: 'sans-serif', textAlign: 'center', marginTop: '50px' }}>
      <h1>Random Number</h1>
      <p style={{ fontSize: '2em' }}>
        { number !== null ? number : "Loading..." }
      </p>
    </div>
  );
}
