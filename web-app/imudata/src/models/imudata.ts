type RawData = {
  id: string;
  timestamp: string;
  data: string;
};

type ProcessedData = {
  repetitions: number;
  spent_time: number;
  total_distance: number;
  spent_energy: number;
};

export type {
  RawData,
  ProcessedData,
};
