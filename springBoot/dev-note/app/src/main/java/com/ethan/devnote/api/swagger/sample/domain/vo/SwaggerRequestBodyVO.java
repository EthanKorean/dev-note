package com.ethan.devnote.api.swagger.sample.domain.vo;

import io.swagger.v3.oas.annotations.Parameter;
import io.swagger.v3.oas.annotations.media.Schema;
import lombok.Builder;
import lombok.Getter;
import lombok.ToString;

@Getter
@Builder
@ToString
public class SwaggerRequestBodyVO {
    @Schema(description = "제목을 입력합니다.", example = "title!")
    private String title;
    @Schema(description = "내용을 입력합니다.", example = "description~")
    private String desc;
    
}
