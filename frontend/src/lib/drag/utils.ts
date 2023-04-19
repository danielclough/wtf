export const calcX = (i:number, viewBoxW: number) => {
    return  (
        i==0
            ? viewBoxW /10
            : i==1
                ? viewBoxW/1.5
                : i==2
                    ? viewBoxW/2.5
                    : i==3
                        ? viewBoxW /10
                        : i==4
                            ? viewBoxW/1.5
                            : i==5
                                ? viewBoxW/2.5
                                : i==6
            ? viewBoxW /10
            : i==7
                ? viewBoxW/1.5
                : i==8
                    ? viewBoxW/2.5
                    : i==9
            ? viewBoxW /10
            : i==10
                ? viewBoxW/1.5
                : i==11
                    ? viewBoxW/2.5
                    : i==12
            ? viewBoxW /10
            : i==13
                ? viewBoxW/1.5
                : i==14
                    ? viewBoxW/2.5
                    : i==15
            ? viewBoxW /10
            : i==16
                ? viewBoxW/1.5
                : i==17
                    ? viewBoxW/2.5
                    : i==18
            ? viewBoxW /10
            : i==19
                ? viewBoxW/1.5
                : i==20
                    ? viewBoxW/2.5
                    : i==21
            ? viewBoxW /10
            : i==22
                ? viewBoxW/1.5
                : i==23
                    ? viewBoxW/2.5
                    : i==24
            ? viewBoxW /10
            : i==25
                ? viewBoxW/1.5
                : i==26
                    ? viewBoxW/2.5
                    : i==27
            ? viewBoxW /10
            : i==28
                ? viewBoxW/1.5
                : i==29
                    ? viewBoxW/2.5
                    : i==31
            ? viewBoxW /10
            : i==32
                ? viewBoxW/1.5
                : i==33
                    ? viewBoxW/2.5
                    : i==34
            ? viewBoxW /10
            : i==35
                ? viewBoxW/1.5
                : i==36
                    ? viewBoxW/2.5
                    : i==37
            ? viewBoxW /10
            : i==38
                ? viewBoxW/1.5
                : viewBoxW/2.5
            )
}

export const calcY = (i: number, viewBoxH: number, calcFontHeight: number) => {
	return i == 0
		? viewBoxH - calcFontHeight * 0
		: i == 1
		? viewBoxH - calcFontHeight * 0
		: i == 2
		? viewBoxH - calcFontHeight * 0
		: i == 3
		? viewBoxH - calcFontHeight * 1
		: i == 4
		? viewBoxH - calcFontHeight * 1
		: i == 5
		? viewBoxH - calcFontHeight * 1
		: i == 6
		? viewBoxH - calcFontHeight * 2
		: i == 7
		? viewBoxH - calcFontHeight * 2
		: i == 8
		? viewBoxH - calcFontHeight * 2
		: i == 9
		? viewBoxH - calcFontHeight * 3
		: i == 10
		? viewBoxH - calcFontHeight * 3
		: i == 11
		? viewBoxH - calcFontHeight * 3
		: i == 12
		? viewBoxH - calcFontHeight * 4
		: i == 13
		? viewBoxH - calcFontHeight * 4
		: i == 14
		? viewBoxH - calcFontHeight * 4
		: i == 15
		? viewBoxH - calcFontHeight * 5
		: i == 16
		? viewBoxH - calcFontHeight * 5
		: i == 17
		? viewBoxH - calcFontHeight * 5
		: i == 18
		? viewBoxH - calcFontHeight * 6
		: i == 19
		? viewBoxH - calcFontHeight * 6
		: i == 20
		? viewBoxH - calcFontHeight * 6
		: i == 21
		? viewBoxH - calcFontHeight * 7
		: i == 22
		? viewBoxH - calcFontHeight * 7
		: i == 23
		? viewBoxH - calcFontHeight * 7
		: i == 24
		? viewBoxH - calcFontHeight * 8
		: i == 25
		? viewBoxH - calcFontHeight * 8
		: i == 26
		? viewBoxH - calcFontHeight * 8
		: i == 27
		? viewBoxH - calcFontHeight * 9
		: i == 28
		? viewBoxH - calcFontHeight * 9
		: i == 29
		? viewBoxH - calcFontHeight * 9
		: i == 30
		? viewBoxH - calcFontHeight * 10
		: i == 31
		? viewBoxH - calcFontHeight * 10
		: i == 32
		? viewBoxH - calcFontHeight * 10
		: i == 33
		? viewBoxH - calcFontHeight * 11
		: i == 34
		? viewBoxH - calcFontHeight * 11
		: i == 35
		? viewBoxH - calcFontHeight * 11
		: i == 36
		? viewBoxH - calcFontHeight * 12
		: i == 37
		? viewBoxH - calcFontHeight * 12
		: viewBoxH - calcFontHeight * 12;
};