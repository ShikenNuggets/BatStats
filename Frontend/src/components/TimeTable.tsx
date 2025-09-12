import React from 'react';

export enum ValueType{
  Seconds = "seconds",
  Count = "count",
  Percent = "percent"
}

type DataPair = [string, number];

interface TimeTableProps{
	data: DataPair[];
	title: string;
	tableKey: string;
	tableValue: string;
  valueType: ValueType;
}

function formatSeconds(seconds: number): string{
  const hrs = Math.floor(seconds / 3600);
  const mins = Math.floor((seconds % 3600) / 60);
  const secs = seconds % 60;

  const pad = (n: number) => n.toString().padStart(2, '0');
  return `${pad(hrs)}:${pad(mins)}:${pad(secs)}`;
}

const TimeTable: React.FC<TimeTableProps> = ({ data, title, tableKey, tableValue, valueType }) => {
  const formatValue = (value: number) => {
    switch(valueType){
      case ValueType.Seconds:
        return formatSeconds(value);
      case ValueType.Percent:
        return `${(value * 100).toFixed(2)}%`;
			case ValueType.Count:
			default:
				return value;
    }
  };

	return(
		<div style={{ textAlign: 'center', paddingTop: '25px' }}>
			{title && <h2 style={{ paddingBottom: '5px' }}>{title}</h2>}
			<table style={{ display: 'inline-block', width: '20%', tableLayout: 'fixed' }} border={1} cellPadding={8}>
		<colgroup>
			<col style={{ width: '10%' }} />
			<col style={{ width: '10%' }} />
		</colgroup>
        <thead>
          <tr>
            <th>{tableKey}</th>
            <th>{tableValue}</th>
          </tr>
        </thead>
        <tbody>
          {
            data.map(([name, value], index) => (
              <tr key={index}>
                <td style={{ textAlign: 'left', padding: '5px' }}>{name}</td>
                <td>{formatValue(value)}</td>
              </tr>
            ))
          }
        </tbody>
      </table>
		</div>
	)
}

export default TimeTable;
