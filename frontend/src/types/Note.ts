export enum Category {
    Personal = "Personal",
    Work = "Work",
    Health = "Health",
    Travel = "Travel",
    Family = "Family",
    Hobby = "Hobby",
    Finance = "Finance",
    Goal = "Goal",
    Memory = "Memory",
    Reflection = "Reflection",
    Unspecified = "Unspecified"
  }
  
  export interface Note {
    id?: number;
    content: string;
    analyzed: boolean;
    category: Category;
    created_at: string;
    updated_at: string;
    analysis?: string;
  }