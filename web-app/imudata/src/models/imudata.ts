type RawData = {
  id: string;
  timestamp: string;
  data: string;
};

type ProcessedData = {
  id: string;
  timestamp: string;
  repetitions: number;
};

export type {
  RawData,
  ProcessedData,
};
